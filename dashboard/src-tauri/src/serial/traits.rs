pub trait SerialSensor {
    /// Name or ID of the sensor
    fn name(&self) -> &str;

    /// Parse an incoming line of text from the serial port
    fn parse_line(&mut self, line: &str) -> bool;

    /// Parse raw incoming bytes from the serial port
    fn parse_bytes(&mut self, bytes: &[u8]) -> bool {
        if let Ok(st) = std::str::from_utf8(bytes) {
            self.parse_line(st.trim())
        } else {
            false
        }
    }
}
