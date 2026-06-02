use super::protocol::{build_frame, LedAction, SerialRequest, FRAME_SET_LED_STATE};

#[derive(Debug, Default, Clone)]
pub struct LedController;

impl LedController {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build_set_led_frame(
        &self,
        wavelength: u8,
        action: LedAction,
        brightness: u8,
    ) -> Vec<u8> {
        let request = SerialRequest::SetLedState {
            wavelength,
            action,
            brightness,
        };
        let payload = vec![wavelength, request_action(&request), brightness];
        build_frame(FRAME_SET_LED_STATE, &payload)
    }
}

fn request_action(request: &SerialRequest) -> u8 {
    match request {
        SerialRequest::SetLedState { action, .. } => action.as_u8(),
        _ => 0,
    }
}
