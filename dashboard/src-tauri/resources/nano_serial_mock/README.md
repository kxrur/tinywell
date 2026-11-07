# Arduino Nano serial simulator

This sketch simulates the mroc payload for dashboard development. It uses USB
serial only; it does not include or require `Wire`, an I²C bus, or the payload.

## Flash and connect

1. Open `nano_serial_mock.ino` in the Arduino IDE.
2. Select the connected Nano board and its port.
3. Upload the sketch.
4. Close the Arduino Serial Monitor so it releases the port.
5. Select the same port in Tinywell and connect at 115200 baud.

The sketch implements the contract in `../serial_protocol_v2.md`, including
polled telemetry, command acknowledgements, a simulated experiment sequence,
and photosensor values affected by LED wavelength/brightness. It emits no
debug text on the serial port.

The simulation models the intended v2 behavior. It is not a substitute for
testing the eventual Pico bridge or physical mroc payload.
