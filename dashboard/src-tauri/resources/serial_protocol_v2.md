# TinyWell Serial Protocol v2

**Contract between the Arduino bridge/simulator and the dashboard application.**

This document supersedes `src-tauri/resources/serial_protocol.md` (v1). It is the
normative wire contract for the Arduino Nano simulator and for any future bridge
(Pico or otherwise) that implements the same interface.

Where v1 disagreed with the mroc payload firmware, this document follows mroc
when mroc's behavior is implemented and intentional. Features that mroc has not
completed are marked explicitly in the conformance section.

---

## 1. Transport

- USB CDC / USB-serial serial.
- Baud: `115200`.
- Format: 8 data bits, no parity, 1 stop bit (8-N-1).
- No hardware or software flow control.
- Requests are sent by the host (dashboard) to the Arduino.
- Responses are sent by the Arduino to the host.
- The Arduino never sends unsolicited frames. All telemetry is host-polled.
- The host sends at most one outstanding request at a time.

### 1.1 Line endings

The host currently appends `0x0A` (`\n`) to each request. The Arduino must ignore
`0x0A` and `0x0D` outside of a frame. Responses are self-delimiting by frame
length and checksum; a trailing newline is optional. The host must ignore a
trailing newline if present.

---

## 2. Frame format

All frames are byte-oriented:

```
| 0xB3 | Frame ID | Payload (0..N bytes) | Checksum |
```

- `0xB3`: start delimiter.
- `Frame ID`: one byte identifying the request or response.
- `Payload`: request arguments or response data.
- `Checksum`: one byte.

### 2.1 Checksum

Sum every byte from the frame ID through the final payload byte, then keep the
least-significant 8 bits:

```
checksum = (frame_id + sum(payload bytes)) & 0xFF
```

The delimiter is not included.

### 2.2 Byte order

All multi-byte numeric fields are little-endian.

### 2.3 Payload limits

- Maximum request payload: 3 bytes.
- Maximum response payload: 57 bytes.
- Maximum frame size: 60 bytes (57-byte payload + ID + delimiter + checksum).

---

## 3. Frame ID summary

| ID | Name | Direction | Request payload | Response |
| --- | --- | --- | --- | --- |
| `0x00` | Ping | Host → Arduino → Host | 0 | `0x00`, 0-byte payload |
| `0x01` | Next experiment phase | Host → Arduino | 0 | `0x85` ack |
| `0x02` | Run experiment phase | Host → Arduino | 1 | `0x85` ack |
| `0x03` | Set LED state | Host → Arduino | 3 | `0x85` ack |
| `0x04` | Test system health | Host → Arduino | 0 | `0x85` ack |
| `0x05` | Set state idle | Host → Arduino | 0 | `0x85` ack |
| `0x55` | Reset | Host → Arduino | 0 | `0x85` ack |
| `0x80` | System status | Host → Arduino | 0 | `0x80`, 5-byte payload |
| `0x81` | Experiment status | Host → Arduino | 0 | `0x81`, 5-byte payload |
| `0x82` | Hardware health | Host → Arduino | 0 | `0x82`, 6-byte payload |
| `0x83` | Environment information | Host → Arduino | 0 | `0x83`, 14-byte payload |
| `0x84` | Photosensor results | Host → Arduino | 0 | `0x84`, 57-byte payload |
| `0x85` | Telecommand acknowledge | Arduino → Host | — | response only |
| `0xE0` | Error response | Arduino → Host | — | response only |

Frame IDs `0x85` and `0xE0` must not be sent as requests. Sending either as a
request produces `0xE0` with `unknown_frame_id`.

---

## 4. Telemetry requests

Telemetry requests have a zero-byte request payload.

### 4.1 Ping (`0x00`)

Liveness check. The Arduino responds locally without consulting the payload.

Request:

```
B3 00 00
```

Response:

```
B3 00 00
```

Checksum is `0x00` because the frame ID is `0x00` and the payload is empty.

### 4.2 System status (`0x80`)

Response payload:

| Field | Type | Meaning |
| --- | --- | --- |
| `fw_and_state` | `u8` | Packed firmware version and live state |
| `uptime` | `u32` | Seconds since payload/bridge boot |

Decode:

```
fw    = (fw_and_state >> 3) & 0x1F
state = fw_and_state & 0x07
```

`state` values:

| Value | State |
| --- | --- |
| `0` | Idle |
| `1` | Preparation |
| `2` | Activation |
| `3` | Growth |
| `4` | Induction |

`fw` is the payload firmware version. Current mroc uses `1`.

Examples:

- `fw=1`, `state=0`, `uptime=0`:

```
B3 80 08 00 00 00 00 88
```

- `fw=3`, `state=1`, `uptime=0x12345678`:

```
B3 80 19 78 56 34 12 AD
```

The second example is a layout fixture; real firmware reports its actual version.

### 4.3 Experiment status (`0x81`)

Response payload:

| Field | Type | Meaning |
| --- | --- | --- |
| `last_state_and_progress` | `u8` | Packed last/current phase and progress |
| `phase_time` | `u32` | Phase start uptime or elapsed duration |

Decode:

```
last_state = (last_state_and_progress >> 3) & 0x1F
progress   = last_state_and_progress & 0x07
```

- `last_state` uses the same phase values as the system status state table.
- `progress`:
  - `0` = incomplete
  - `1` = complete
- `phase_time` semantics:
  - While a phase is active: the uptime, in seconds, at which that phase started.
  - After a phase completes, or after a forced idle: the elapsed duration, in
    seconds, of the most recent phase.

Forcing idle may leave `progress` at `0` if the phase was interrupted; see
section 7.

Example (`last_state=2`, `progress=1`, `phase_time=42`):

```
B3 81 11 2A 00 00 00 BC
```

### 4.4 Hardware health (`0x82`)

Response payload:

| Field | Type | Meaning |
| --- | --- | --- |
| `bipump_current` | `u16` | Raw ADC reading for the mixing (bipump) current sense |
| `heater_current` | `u16` | Raw ADC reading for the heater current sense |
| `power_supply_sense` | `u16` | Raw ADC reading for the power-supply sense |

These are raw ADC values, nominally 12-bit (`0..4095`). No engineering-unit
conversion is defined at the serial layer.

Legacy naming note: the app's existing Rust/JSON fields are named
`pump_current`, `heater_current`, and `vref_current`. The first is the bipump
sense, not the feed pump; the third is the power-supply sense, not a
voltage-reference current. The wire layout is unchanged.

Example (`300`, `400`, `500`):

```
B3 82 2C 01 90 01 F4 01 35
```

### 4.5 Environment information (`0x83`)

Response payload:

| Field | Type | Meaning |
| --- | --- | --- |
| `well_temp` | `u16` | Well temperature in whole degrees Celsius |
| `ambient_temp` | `i32` | Ambient temperature in 0.01 °C |
| `ambient_pressure` | `u32` | Ambient pressure in BME280 Q24.8 Pa |
| `ambient_humidity` | `u32` | Ambient humidity in BME280 Q22.10 %RH |

Conversions:

```
well_temp_c          = well_temp
ambient_temp_c       = ambient_temp / 100.0
ambient_pressure_pa  = ambient_pressure / 256.0
ambient_pressure_hpa = ambient_pressure / 25600.0
ambient_humidity_pct = ambient_humidity / 1024.0
```

**Pressure units:** v1 incorrectly described `ambient_pressure` as already being
in Pascals. The BME280 integer compensation used by mroc produces Q24.8 Pa, and
mroc stores that raw value without shifting. For example, `101325 Pa` is stored
as `25939200` (`0x018BCD00`). The app must use the Q24.8 conversions above.

`well_temp` is derived from `LMT01_Read()` and truncated/cast to `u16` by
firmware.

Example:

- `well_temp = 25`
- `ambient_temp = 2300` (23.00 °C)
- `ambient_pressure = 25_939_200` (101325 Pa in Q24.8)
- `ambient_humidity = 46_080` (45.0 %RH)

```
B3 83 19 00 FC 08 00 00 00 CD 8B 01 00 B4 00 00 AD
```

### 4.6 Photosensor results (`0x84`)

Response payload:

| Field | Type | Meaning |
| --- | --- | --- |
| `wavelength` | `u8` | Instrument wavelength enum |
| `v0..v13` | `u32 × 14` | Integer-truncated lux estimate for each well |

Wavelength enum:

| Value | Wavelength |
| --- | --- |
| `0` | 470 nm |
| `1` | 570 nm |
| `2` | 630 nm |
| `3` | 850 nm |

Payload length is `1 + 14 × 4 = 57` bytes. Values may be the most recent capture
and are not guaranteed to update while no acquisition is running.

Example with `wavelength=0` and `vN = 1000 + 123 × N`:

```
B3 84 00 E8 03 00 00 63 04 00 00 DE 04 00 00 59 05 00 00 D4 05 00 00 4F 06 00 00 CA 06 00 00 45 07 00 00 C0 07 00 00 3B 08 00 00 B6 08 00 00 31 09 00 00 AC 09 00 00 27 0A 00 00 48
```

---

## 5. Telecommands

All telecommands are acknowledged with a `0x85` frame. The request payload is
listed per command; the frame ID identifies the command.

The host must wait for the acknowledgement before sending another telecommand.
The recommended host timeout is 1500 ms.

### 5.1 Next experiment phase (`0x01`)

Request payload: none.

Semantics:

- Valid only when the system is idle (`state == 0`).
- Advances to the next experiment phase.
- If the system is not idle, respond with `not_in_idle_mode` (`0x01`).
- The host should not send this when already in the final phase.

Request:

```
B3 01 01
```

Success acknowledgement:

```
B3 85 01 00 86
```

Not-idle acknowledgement:

```
B3 85 01 01 87
```

### 5.2 Run experiment phase (`0x02`)

Request payload:

| Field | Type | Range |
| --- | --- | --- |
| `phase` | `u8` | `1..4` |

Semantics:

- Valid only when the system is idle (`state == 0`).
- Sets the selected phase directly.
- Phase `0` is not a valid argument; use `Set state idle` (`0x05`) instead.
- A phase outside `1..4` produces a `0x85` acknowledgement with
  `tc_result = unknown_experiment_phase` (`0x03`).
- If the system is not idle, respond with `not_in_idle_mode` (`0x01`).

Request for phase 2:

```
B3 02 02 04
```

Success acknowledgement:

```
B3 85 02 00 87
```

Not-idle acknowledgement:

```
B3 85 02 01 88
```

Unknown-phase acknowledgement (phase 5):

```
B3 85 02 03 8A
```

### 5.3 Set LED state (`0x03`)

Request payload:

| Field | Type | Range | Meaning |
| --- | --- | --- | --- |
| `wavelength` | `u8` | `0..3` | Instrument wavelength enum |
| `action` | `u8` | `0..3` | `0` off, `1` on, `2` toggle, `3` set brightness |
| `brightness` | `u8` | `0..255` | Only used when `action == 3` |

Semantics:

- Applies to the selected wavelength channel, which drives all wells.
- `brightness` is ignored for actions other than `3`.
- Out-of-range values produce `0xE0` with `invalid_args` (`0x03`).
- There is no LED state telemetry. The host must not assume hardware state from
  a successful acknowledgement alone.

Example: wavelength `0`, action `3`, brightness `128`:

```
B3 03 00 03 80 86
```

Success acknowledgement:

```
B3 85 03 00 88
```

### 5.4 Test system health (`0x04`)

Request payload: none.

Semantics:

- Requests a payload health test.
- Acknowledged with `0x85`.
- Current mroc accepts this command but performs no action. No additional result
  payload is defined.

Request:

```
B3 04 04
```

Success acknowledgement:

```
B3 85 04 00 89
```

### 5.5 Set state idle (`0x05`)

Request payload: none.

Semantics:

- Stops the pumps, closes the valve, turns off all LEDs, and returns the payload
  to idle. Current mroc does not explicitly call `Heater_Off()` during this
  transition; the heater driver is otherwise unused.
- This is the recommended stop action.
- Always accepted when the system is reachable.

Request:

```
B3 05 05
```

Success acknowledgement:

```
B3 85 05 00 8A
```

### 5.6 Reset (`0x55`)

Request payload: none.

Semantics:

- Requests a payload reset.
- Acknowledged with `0x85`.
- Current mroc accepts this command but performs no reset. The reference Arduino
  simulator resets its simulated state and uptime.

Request:

```
B3 55 55
```

Success acknowledgement:

```
B3 85 55 00 DA
```

---

## 6. Telecommand acknowledgement (`0x85`)

Response payload:

| Field | Type | Meaning |
| --- | --- | --- |
| `tc_id` | `u8` | Frame ID of the telecommand being acknowledged |
| `tc_result` | `u8` | Result code |

Result codes:

| Value | Name | Meaning |
| --- | --- | --- |
| `0x00` | `success` | Command accepted/executed |
| `0x01` | `not_in_idle_mode` | Command requires idle mode |
| `0x02` | `unknown_command` | Command not recognized by the payload |
| `0x03` | `unknown_experiment_phase` | Requested phase is not valid |
| `0x04` | `too_many_arguments` | Payload received too many arguments |
| `0x05` | `too_few_arguments` | Payload received too few arguments |
| `0x06` | `timeout` | Payload communication timed out |

`0x07..0xFF` are reserved and must not be used by the host. v1 listed
`0x07 = invalid_argument`; that code does not exist in mroc.

Protocol-level argument validation errors use the `0xE0` error response instead
of a `0x85` result. Invalid LED wavelength/action arguments use
`invalid_args` (`0x03`); an unknown run-phase value uses the telecommand result
`unknown_experiment_phase` (`0x03`).

---

## 7. Command and state semantics

### 7.1 States

| Value | State | Behavior |
| --- | --- | --- |
| `0` | Idle | No experiment actions; actuators and LEDs off after an orderly stop |
| `1` | Preparation | Mixing pump runs for 20 s, then the phase completes and returns to idle |
| `2` | Activation | Valve opens and feed pump runs for 120 s, then the phase completes, the valve closes, and the system returns to idle; acquisition wavelength becomes 630 nm |
| `3` | Growth | LEDs alternate between 630 nm and 850 nm; sensor captures repeat until stopped |
| `4` | Induction | 470 nm LED is used for repeated sensor captures until stopped |

Growth and induction do not complete automatically. Use `0x05` to stop them.

### 7.2 Interlocks

- `0x01` and `0x02` are accepted only when `state == 0`.
- `0x05` is always accepted.
- `0x04` and `0x55` are accepted but are currently no-ops in mroc; see the
  conformance section.

### 7.3 Status fields

- System status `state` is the **live** state.
- Experiment status `last_state` is the most recent/current experiment phase.
- `progress` describes whether that phase completed.
- `phase_time` is a start uptime while active, or an elapsed duration after
  completion/forced idle.
- Forcing idle aborts the current phase; it does not guarantee
  `progress == complete`.

---

## 8. Error response (`0xE0`)

Response payload:

| Field | Type | Meaning |
| --- | --- | --- |
| `error_code` | `u8` | Error code |

Error codes:

| Value | Name | Meaning |
| --- | --- | --- |
| `0x01` | `unknown_frame_id` | Frame ID is not supported as a request |
| `0x02` | `invalid_length` | Request payload length is wrong |
| `0x03` | `invalid_args` | Argument is out of range |
| `0x04` | `i2c_read_failed` | Bridge could not read from the payload |
| `0x05` | `i2c_write_failed` | Bridge could not write to the payload |
| `0x06` | `ack_failed` | Bridge could not read the telecommand acknowledgement |

The Arduino simulator does not use `0x04`–`0x06` unless a fault-injection mode is
enabled. A real bridge may return them.

Example unknown frame ID:

```
B3 E0 01 E1
```

---

## 9. Validation rules

The Arduino must enforce the following:

- Request payload lengths are defined by the frame ID tables in this document.
- Unknown request frame IDs produce `0xE0 0x01`.
- If a malformed frame's checksum does not match, it is discarded silently. A
  receiver may return `0xE0 0x02` when it can positively detect an invalid
  length, but extra or missing bytes are not always distinguishable from a
  checksum error in this framing scheme.
- Invalid LED wavelength/action arguments produce `0xE0 0x03`; an invalid run
  phase produces a `0x85` acknowledgement with `tc_result = 0x03`.
- Checksum mismatches cause the frame to be discarded silently; the host will
  time out. No partial frame is processed.
- Bytes before `0xB3` are ignored.
- `0x0A` and `0x0D` outside a frame are ignored.
- Only one response is sent per valid request.
- No unsolicited frames are sent.

Recommended host behavior:

- Use a 1500 ms timeout per request.
- Use `Ping` as a device-ready check after opening the port.
- Poll telemetry no faster than necessary; 1–2 Hz per route is sufficient.

---

## 10. Conformance and known gaps

This section is informative; the normative contract is sections 1–9.

### 10.1 Current mroc payload

| Feature | Status |
| --- | --- |
| `0x80` system status | Implemented and valid |
| `0x81` experiment status | Implemented and valid |
| `0x82` hardware health | Implemented and valid; raw ADC |
| `0x83` environment | Implemented; pressure is Q24.8 as documented here |
| `0x84` photosensor | Implemented and valid |
| `0x01` next phase | Implemented |
| `0x02` run phase | Implemented, but does not validate phase range |
| `0x03` set LED | **Incomplete stub**; does not drive the LED hardware |
| `0x04` test health | Accepted, no-op |
| `0x05` set idle | Implemented |
| `0x55` reset | Accepted, no-op |
| `0x85` success result | `TC_SUCCESS` is never emitted by mroc; failure/timeout paths only |

### 10.2 Current Pico bridge

- Serves `0x00`, `0x80`, `0x83`, `0x84`, and `0x03`.
- `0x81` and `0x82` are accepted by the parser but not handled; they currently
  return `unknown_frame_id`.
- Does not forward `0x01`, `0x02`, `0x04`, `0x05`, or `0x55`.
- Does not append a newline to responses; the app tolerates this.
- **Not updated in this pass.** It should be aligned to this document later.

### 10.3 Arduino Nano simulator

The simulator must implement the full normative contract:

- No I²C, no `Wire`.
- All telemetry routes and telecommands.
- Simulated experiment state machine and plausible sensor data.
- Success acknowledgements as specified.
- LED commands visibly affect photosensor telemetry.
- Optional compile-time fault injection for testing error paths.

### 10.4 Changes from v1

1. Corrected ambient pressure to BME280 Q24.8, not Pa.
2. Corrected hardware-health field meanings: bipump current and power-supply
   sense.
3. Added telecommands `0x01`, `0x02`, `0x04`, `0x05`, and `0x55`.
4. Removed `0x07 invalid_argument`.
5. Clarified success acknowledgements and distinguished `0x85` results from
   `0xE0` protocol errors.
6. Clarified that newlines are optional and ignored.
7. Documented current mroc and Pico gaps explicitly.

---

## Appendix A — Reference simulator behavior

This appendix describes the required behavior of the Nano simulator used for
dashboard testing. It is not part of the wire contract.

- Phase timings match section 7.1: preparation 20 s, activation 120 s.
- `0x05` stops all simulated actuators and LEDs.
- `0x55` resets state and uptime.
- Simulated environment values drift smoothly and remain in plausible ranges.
- Simulated photosensor values:
  - use a stable per-well pattern;
  - scale with active wavelength and LED brightness;
  - fall to dark/noise levels when all LEDs are off;
  - use deterministic noise so test runs are repeatable.
- Simulated hardware health:
  - 12-bit raw ADC values;
  - bipump reading rises while the simulated mixing pump runs;
  - heater reading reflects simulated heater duty;
  - power-supply reading is stable with small noise.
- Optional fault-injection compile flags may disable a telemetry route, reject a
  command, or force an error frame. These hooks must not be reachable through
  normal app traffic.

---

## Appendix B — Quick checksum reference

All examples use:

```
checksum = (frame_id + sum(payload)) & 0xFF
```

| Frame | Bytes |
| --- | --- |
| Ping request/response | `B3 00 00` |
| System status request | `B3 80 80` |
| Experiment status request | `B3 81 81` |
| Hardware health request | `B3 82 82` |
| Environment request | `B3 83 83` |
| Photosensor request | `B3 84 84` |
| Next phase request | `B3 01 01` |
| Next phase success ack | `B3 85 01 00 86` |
| Run phase 2 request | `B3 02 02 04` |
| Run phase 2 success ack | `B3 85 02 00 87` |
| Set LED request | `B3 03 00 03 80 86` |
| Set LED success ack | `B3 85 03 00 88` |
| Test health request | `B3 04 04` |
| Test health success ack | `B3 85 04 00 89` |
| Set idle request | `B3 05 05` |
| Set idle success ack | `B3 85 05 00 8A` |
| Reset request | `B3 55 55` |
| Reset success ack | `B3 85 55 00 DA` |
| Unknown frame error | `B3 E0 01 E1` |
