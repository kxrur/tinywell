# Pico Serial Protocol

This document describes a framed, hex-oriented serial protocol for the Pico firmware in
[arduino/pico_msp430_i2c.ino](arduino/pico_msp430_i2c.ino). The payloads map directly to
the MSP430 telemetry and telecommand data used in this repository.

## Transport

- USB CDC serial
- Baud: 115200
- Format: 8-N-1
- Line ending: `\n` (CR `\r` is ignored)

## Frame Structure

All frames are byte-oriented and use a start delimiter and checksum similar to the
Battery Cell Bench Protocol.

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

| Frame ID | Description                          | Direction              | Payload Length |
| -------- | ------------------------------------ | ---------------------- | -------------- |
| 0x00     | Ping                                 | Host->Pico, Pico->Host | 0              |
| 0x80     | System status request                | Host->Pico             | 0              |
| 0x81     | Experiment status request (reserved) | Host->Pico             | 0              |
| 0x82     | Hardware health request (reserved)   | Host->Pico             | 0              |
| 0x83     | Environment info request             | Host->Pico             | 0              |
| 0x84     | Photosensor results request          | Host->Pico             | 0              |
| 0x03     | Set LED state                        | Host->Pico             | 3              |
| 0x85     | Telecommand acknowledge              | Pico->Host             | 2              |
| 0xE0     | Error response                       | Pico->Host             | 1              |

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

### Environment Information (0x83)

**Request**

```
| 0xB3 | 0x83 | Checksum |
```

**Response Payload**

```
| well_temp (u16) | ambient_temp (i32) | ambient_pressure (u32) | ambient_humidity (u32) |
```

All fields are raw MSP430 units.

**Response Frame**

```
| 0xB3 | 0x83 | well_temp[0..1] | ambient_temp[0..3] | ambient_pressure[0..3] | ambient_humidity[0..3] | Checksum |
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

**Response Frame**

```
| 0xB3 | 0x84 | wavelength | v0[0..3] | v1[0..3] | ... | v13[0..3] | Checksum |
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
- `brightness`: 0..255 (used only when `action = 3`)

**Request Frame**

```
| 0xB3 | 0x03 | wavelength | action | brightness | Checksum |
```

**Response (Telecommand Acknowledge 0x85)**

```
| 0xB3 | 0x85 | tc_id (u8) | tc_result (u8) | Checksum |
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

## Notes

- The request/response payloads mirror the MSP430 I2C telemetry and telecommand fields.
- All numeric fields are binary, little-endian.
- This document specifies the framed protocol; the current firmware must be updated to implement it.
