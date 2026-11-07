# Serial hardware tests

`serial_hardware.rs` exercises the v2 host-request routes against a connected
serial device. The recommended target is the Nano simulator in
`../resources/nano_serial_mock/`, which implements the full v2 contract without
the payload. The tests are ignored by default and are never run by a normal
`cargo test` invocation.

The current mroc/Pico path does not yet implement every v2 route or successful
telecommand acknowledgements. The command tests are therefore intended for the
Nano simulator until the Pico and payload are aligned with the contract.

## Before running

1. Close Tinywell and any terminal or application that may already own the serial port.
2. Flash `nano_serial_mock.ino` to an Arduino Nano and connect it over USB, or
   connect another device implementing protocol v2.
3. Identify its port, such as `/dev/ttyUSB0` on Linux or `COM3` on Windows.
4. On Linux, ensure the current user has permission to open the port (commonly through the
   `dialout` group).

The suite includes stateful command tests. Use the Nano simulator rather than a
payload attached to an experiment. The LED test sets wavelength `0` brightness
to `0`, and the phase tests start and stop simulated phases.

Opening some USB serial devices resets them. The test waits for a Ping response for up to ten
seconds before exercising each route.

## Run the complete v2 suite

From `src-tauri`:

```bash
TINYWELL_SERIAL_PORT=/dev/ttyUSB0 \
  cargo test --test serial_hardware -- --ignored --nocapture --test-threads=1
```

On PowerShell:

```powershell
$env:TINYWELL_SERIAL_PORT = "COM3"
cargo test --test serial_hardware -- --ignored --nocapture --test-threads=1
```

`--test-threads=1` is required so tests never contend for the physical serial port.

## Run only the LED route

This test turns wavelength `0` brightness off. It is ignored by default:

```bash
TINYWELL_SERIAL_PORT=/dev/ttyUSB0 \
  cargo test --test serial_hardware set_led_state_route_acknowledges -- --ignored --nocapture --test-threads=1
```
