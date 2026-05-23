#[cfg(test)]
mod tests {
    use crate::model::grid::Led;
    use crate::serial::led::{LedController, CMD_SET_LED};

    #[test]
    fn test_led_controller_creation() {
        let controller = LedController::new();
        // Currently it's an empty struct, but verifying it constructs properly and implements Default
        let _controller_default = LedController::default();
    }

    #[test]
    fn test_led_build_payload() {
        let controller = LedController::new();

        let mut led = Led::default();
        led.id = 5;
        led.enabled = true;
        led.brightness = 0.5; // Will map to ~128
        led.wavelength = 450; // 0x01C2

        let payload = controller.build_payload(&led);

        assert_eq!(payload.len(), 6);
        assert_eq!(payload[0], CMD_SET_LED); // 0x10
        assert_eq!(payload[1], 5); // id
        assert_eq!(payload[2], 1); // enabled flag
        assert_eq!(payload[3], 128); // brightness 0.5 * 255.0 = 127.5 -> round = 128
        assert_eq!(payload[4], 0x01); // wavelength high byte
        assert_eq!(payload[5], 0xC2); // wavelength low byte
    }

    #[test]
    fn test_led_build_payload_disabled() {
        let controller = LedController::new();

        let mut led = Led::default();
        led.id = 250;
        led.enabled = false;
        led.brightness = 1.0; // 255
        led.wavelength = 1000; // 0x03E8

        let payload = controller.build_payload(&led);

        assert_eq!(payload[1], 250);
        assert_eq!(payload[2], 0); // disabled
        assert_eq!(payload[3], 255); // brightness max
        assert_eq!(payload[4], 0x03); // high byte
        assert_eq!(payload[5], 0xE8); // low byte
    }
}
