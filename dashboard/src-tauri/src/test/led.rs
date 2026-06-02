#[cfg(test)]
mod tests {
    use crate::serial::led::LedController;
    use crate::serial::protocol::{LedAction, FRAME_SET_LED_STATE};

    #[test]
    fn test_led_controller_creation() {
        let controller = LedController::new();
        // Currently it's an empty struct, but verifying it constructs properly and implements Default
        let _controller_default = LedController::default();
    }

    #[test]
    fn test_led_build_frame() {
        let controller = LedController::new();

        let frame = controller.build_set_led_frame(2, LedAction::SetBrightness, 128);

        assert_eq!(frame[0], 0xB3);
        assert_eq!(frame[1], FRAME_SET_LED_STATE);
        assert_eq!(frame[2], 2);
        assert_eq!(frame[3], 3);
        assert_eq!(frame[4], 128);
    }

    #[test]
    fn test_led_build_frame_disabled() {
        let controller = LedController::new();

        let frame = controller.build_set_led_frame(1, LedAction::Off, 0);

        assert_eq!(frame[0], 0xB3);
        assert_eq!(frame[1], FRAME_SET_LED_STATE);
        assert_eq!(frame[2], 1);
        assert_eq!(frame[3], 0);
        assert_eq!(frame[4], 0);
    }
}
