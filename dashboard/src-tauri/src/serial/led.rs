use super::traits::SerialSensor;
use crate::model::grid::Led;
use crate::serial::Serial;
use std::time::Duration;

/// Command byte for setting an LED state
pub const CMD_SET_LED: u8 = 0x10;

#[derive(Debug, Default, Clone)]
pub struct LedController {}

impl LedController {
    pub fn new() -> Self {
        Self::default()
    }

    /// Serialize an LED's properties into a byte payload for the serial port
    pub fn build_payload(&self, led: &Led) -> Vec<u8> {
        let mut payload = Vec::with_capacity(6);

        // Command identifier
        payload.push(CMD_SET_LED);

        // LED ID (Assuming max 255 LEDs. If you need more, use 2 bytes like wavelength)
        payload.push((led.id & 0xFF) as u8);

        // Enabled flag (1 or 0)
        payload.push(if led.enabled { 1 } else { 0 });

        // Brightness 0.0-1.0 mapped to 0-255
        let brightness_byte = (led.brightness.clamp(0.0, 1.0) * 255.0).round() as u8;
        payload.push(brightness_byte);

        // Wavelength is u16 (split into Big-Endian high and low bytes)
        payload.push((led.wavelength >> 8) as u8);
        payload.push((led.wavelength & 0xFF) as u8);

        payload
    }

    /// Send a raw byte payload to the specified serial port
    pub fn send_payload(&self, port_name: &str, buf: &[u8]) {
        if let Ok(mut port) = serialport::new(port_name, 115_200)
            .timeout(Duration::from_millis(10))
            .open()
        {
            let _ = port.write(buf);
        }
    }

    /// Helper function to build the payload and send it over the active serial port
    pub fn update_led(&self, serial: &Serial, led: &Led) {
        if let Some(port_name) = serial.get_active_port_name() {
            let payload = self.build_payload(led);
            self.send_payload(&port_name, &payload);
        }
    }
}

impl SerialSensor for LedController {
    fn name(&self) -> &str {
        "LED_CONTROLLER"
    }

    fn parse_line(&mut self, _line: &str) -> bool {
        todo!("Handle incomeing confiration of LED change")
    }
}
