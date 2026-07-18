# Serial hardware tests

`serial_hardware.rs` exercises every host-request route documented in
`../resources/serial_protocol.md` against a physically connected Tinywell device.
The tests are ignored by default and are never run by a normal `cargo test` invocation.

## Before running

1. Close Tinywell and any terminal or application that may already own the serial port.
2. Connect the USB device.
3. Identify its port, such as `/dev/ttyUSB0` on Linux or `COM3` on Windows.
4. On Linux, ensure the current user has permission to open the port (commonly through the
   `dialout` group).

The standard suite performs read-only requests. Run the LED test separately because it sets
wavelength `0` brightness to `0`.

Opening some USB serial devices resets them. The test waits for a Ping response for up to ten
seconds before exercising each route.

## Run read-only routes

From `src-tauri`:

```bash
TINYWELL_SERIAL_PORT=/dev/ttyUSB0 \
  cargo test --test serial_hardware -- --ignored --skip set_led_state_route_acknowledges --nocapture --test-threads=1
```

On PowerShell:

```powershell
$env:TINYWELL_SERIAL_PORT = "COM3"
cargo test --test serial_hardware -- --ignored --skip set_led_state_route_acknowledges --nocapture --test-threads=1
```

`--test-threads=1` is required so tests never contend for the physical serial port.

## Include the LED route

Only run this when changing the LED state is safe. The test is ignored, so include
`--ignored` explicitly:

```bash
TINYWELL_SERIAL_PORT=/dev/ttyUSB0 \
  cargo test --test serial_hardware set_led_state_route_acknowledges -- --ignored --nocapture --test-threads=1
```
