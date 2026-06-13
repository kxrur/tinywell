# Pico Serial Protocol

This document describes a framed, hex-oriented serial protocol. The payloads map directly to
the MSP430 telemetry and telecommand data used in this repository.

## Transport

- USB CDC serial
- Baud: 115200
- Format: 8-N-1
- Line ending: `\n` (CR `\r` is ignored)

## Frame Structure

All frames are byte-oriented and use a start delimiter and checksum.

```
| 0xB3 | Frame ID | Payload (0..N bytes) | Checksum |
```

- **0xB3**: start delimiter
- **Frame ID**: command/response identifier
- **Payload**: command arguments or telemetry data
- **Checksum**: 8-bit checksum over all bytes **except** the delimiter

**Checksum definition**

- Sum all bytes from `Frame ID` through the final payload byte.
- Keep the least-significant 8 bits (mod 256).

```
checksum = (sum(frame_id + payload bytes)) & 0xFF
```

All multi-byte numeric fields are **little-endian** unless noted.

## Request/Response Direction

- Requests are sent from the host to the Pico.
- Responses are sent from the Pico to the host.
- If a request fails, the Pico responds with an `ERR` frame (see below).

## Frame IDs

These IDs mirror the MSP430 command IDs where possible to avoid translation.

| Frame ID | Description                 | Direction              | Payload Length |
| -------- | --------------------------- | ---------------------- | -------------- |
| 0x00     | Ping                        | Host->Pico, Pico->Host | 0              |
| 0x80     | System status request       | Host->Pico             | 0              |
| 0x81     | Experiment status request   | Host->Pico             | 0              |
| 0x82     | Hardware health request     | Host->Pico             | 0              |
| 0x83     | Environment info request    | Host->Pico             | 0              |
| 0x84     | Photosensor results request | Host->Pico             | 0              |
| 0x03     | Set LED state               | Host->Pico             | 3              |
| 0x85     | Telecommand acknowledge     | Pico->Host             | 2              |
| 0xE0     | Error response              | Pico->Host             | 1              |

## Requests and Responses

### Ping (0x00)

**Request**

```
| 0xB3 | 0x00 | Checksum |
```

**Response**

```
| 0xB3 | 0x00 | Checksum |
```

**Example Response**

```
| 0xB3 | 0x00 | 0x00 |
```

### System Status (0x80)

**Request**

```
| 0xB3 | 0x80 | Checksum |
```

**Response Payload**

```
| fw_and_state (u8) | uptime (u32) |
```

`fw_and_state` packs firmware version and system state:

- `fw = (fw_and_state >> 3) & 0x1F`
- `state = fw_and_state & 0x07`

**Response Frame**

```
| 0xB3 | 0x80 | fw_and_state | uptime[0..3] | Checksum |
```

**Example Response**

Example below uses `fw = 3`, `state = 1`, and `uptime = 0x12345678` seconds.

```
| 0xB3 | 0x80 | 0x19 | 0x78 | 0x56 | 0x34 | 0x12 | 0xAD |
```

Decoded:

- `fw_and_state = 0x19` -> `fw = 3`, `state = 1`
- `uptime = 0x12345678`

### Experiment Status (0x81)

**Request**

```
| 0xB3 | 0x81 | Checksum |
```

The payload for this route is:

```
| last_state_and_progress (u8) | elapsed_or_started_at_uptime (u32) |
```

Where:

- `last_state = (last_state_and_progress >> 3) & 0x1F`
- `progress = last_state_and_progress & 0x07`

- `last_state` is one of `STATE_IDLE=0`, `STATE_PREPARATION=1`, `STATE_ACTIVATION=2`, `STATE_GROWTH=3`, `STATE_INDUCTION=4`
- `progress` is currently only `0` (`STATE_INCOMPLETE`) or `1` (`STATE_COMPLETE`)
- the `u32` field is not always a pure duration:
  - while a phase is active, it stores the uptime at which that phase started
  - after completion or after forcing idle, it stores the elapsed duration for the most recent phase

Example target response payload for `last_state = 2`, `progress = 1`, `u32 = 42`:

```
| 0xB3 | 0x81 | 0x11 | 0x2A | 0x00 | 0x00 | 0x00 | 0xBC |
```

### Hardware Health (0x82)

**Request**

```
| 0xB3 | 0x82 | Checksum |
```

The payload for this route is:

```
| pump_current (u16) | heater_current (u16) | vref_current (u16) |
```

These are raw ADC readings. The first field is populated from the
`bipumpSense` channel even though the register field is named `pump_current_sensing`.

Example target response payload for `pump = 300`, `heater = 400`, `vref = 500`:

```
| 0xB3 | 0x82 | 0x2C | 0x01 | 0x90 | 0x01 | 0xF4 | 0x01 | 0x35 |
```

### Environment Information (0x83)

**Request**

```
| 0xB3 | 0x83 | Checksum |
```

**Response Payload**

```
| well_temp (u16) | ambient_temp (i32) | ambient_pressure (u32) | ambient_humidity (u32) |
```

Field semantics:

- `well_temp`: Celsius value derived from the LMT01 pulse-count output, then truncated/cast into `u16` by firmware
- `ambient_temp`: compensated BME280 temperature integer
- `ambient_pressure`: compensated BME280 pressure integer
- `ambient_humidity`: compensated BME280 humidity integer

More specifically:

- `LMT01_Read()` converts the LMT01 pulse count into Celsius and the result is truncated when stored into the `u16` register field
- `ambient_temp` is stored in centi-degrees Celsius
- `ambient_pressure` is stored in Pascals
- `ambient_humidity` is stored in BME280 fixed-point humidity units where `1024 = 1 %RH`

Display conversions:

- `well_temp_c = well_temp`
- `ambient_temp_c = ambient_temp / 100`
- `ambient_pressure_hpa = ambient_pressure / 100`
- `ambient_humidity_percent = ambient_humidity / 1024`

Bosch BME280 datasheet traceability:

[doc](https://gebrabit.com/wp-content/uploads/2024/09/GebraBit-BME280-Datasheet.pdf)

- Temperature: page 25 states the compensated integer output has `0.01 DegC` resolution and gives `5123 -> 51.23 DegC`
- Pressure: page 25 states the compensated pressure output is in `Pa`; page 50 gives `96386 -> 96386 Pa = 963.86 hPa`
- Humidity: page 25 states the compensated humidity output is `%RH` in `Q22.10` format and gives `47445 / 1024 = 46.333 %RH`

**Response Frame**

```
| 0xB3 | 0x83 | well_temp[0..1] | ambient_temp[0..3] | ambient_pressure[0..3] | ambient_humidity[0..3] | Checksum |
```

**Example Response**

Example below is a byte-layout fixture:

- `well_temp = 250`
- `ambient_temp = 23000`
- `ambient_pressure = 101325`
- `ambient_humidity = 45000`

```
| 0xB3 | 0x83 | 0xFA | 0x00 | 0xD8 | 0x59 | 0x00 | 0x00 | 0xCD | 0x8B | 0x01 | 0x00 | 0xC8 | 0xAF | 0x00 | 0x00 | 0x7E |
```

### Photosensor Results (0x84)

**Request**

```
| 0xB3 | 0x84 | Checksum |
```

**Response Payload**

```
| wavelength (u8) | v0 (u32) | v1 (u32) | ... | v13 (u32) |
```

- `wavelength` is the `Exp_InstrumentWavelength` enum: `0=W470NM`, `1=W570NM`, `2=W630NM`, `3=W850NM`
- each `vN` is the integer-truncated lux estimate returned by `OPT4003_Read()`

**Response Frame**

```
| 0xB3 | 0x84 | wavelength | v0[0..3] | v1[0..3] | ... | v13[0..3] | Checksum |
```

**Example Response**

Example below uses `wavelength = 0` and `vN = 1000 + (N * 123)`:

```
| 0xB3 | 0x84 | 0x00 | 0xE8 | 0x03 | 0x00 | 0x00 | 0x63 | 0x04 | 0x00 | 0x00 | 0xDE | 0x04 | 0x00 | 0x00 | 0x59 | 0x05 | 0x00 | 0x00 | 0xD4 | 0x05 | 0x00 | 0x00 | 0x4F | 0x06 | 0x00 | 0x00 | 0xCA | 0x06 | 0x00 | 0x00 | 0x45 | 0x07 | 0x00 | 0x00 | 0xC0 | 0x07 | 0x00 | 0x00 | 0x3B | 0x08 | 0x00 | 0x00 | 0xB6 | 0x08 | 0x00 | 0x00 | 0x31 | 0x09 | 0x00 | 0x00 | 0xAC | 0x09 | 0x00 | 0x00 | 0x27 | 0x0A | 0x00 | 0x00 | 0x48 |
```

### Set LED State (0x03)

**Request Payload**

```
| wavelength (u8) | action (u8) | brightness (u8) |
```

- `wavelength`: instrument wavelength enum (0..3)
- `action`:
  - `0` = off
  - `1` = on
  - `2` = toggle
  - `3` = set brightness
- `brightness`: 0..255

`brightness` is only consumed when `action = 3`; for the other
actions it is accepted but ignored.

**Request Frame**

```
| 0xB3 | 0x03 | wavelength | action | brightness | Checksum |
```

**Response (Telecommand Acknowledge 0x85)**

```
| 0xB3 | 0x85 | tc_id (u8) | tc_result (u8) | Checksum |
```

`tc_result` values:

- `0x00` = success
- `0x01` = not_in_idle_mode
- `0x02` = unknown_command
- `0x03` = unknown_experiment_phase
- `0x04` = too_many_arguments
- `0x05` = too_few_arguments
- `0x06` = timeout
- `0x07` = invalid_argument

**Example Response**

Example acknowledgement for a successful `Set LED State` command:

```
| 0xB3 | 0x85 | 0x03 | 0x00 | 0x88 |
```

## Error Response (0xE0)

If the Pico cannot parse a frame or fulfill a request, it responds with:

```
| 0xB3 | 0xE0 | error_code (u8) | Checksum |
```

Error codes:

- `0x01` = unknown_frame_id
- `0x02` = invalid_length
- `0x03` = invalid_args
- `0x04` = i2c_read_failed
- `0x05` = i2c_write_failed
- `0x06` = ack_failed

**Example Response**

Example unknown frame ID error:

```
| 0xB3 | 0xE0 | 0x01 | 0xE1 |
```

## Notes

- The request/response payloads mirror the MSP430 I2C telemetry and telecommand fields.
- All numeric fields are binary, little-endian.
