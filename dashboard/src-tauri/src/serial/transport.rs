use serialport::SerialPort;
use std::time::Duration;

use super::errors::SerialError;

pub trait SerialTransport {
    fn open(&self, port_name: &str, baud_rate: u32) -> Result<Box<dyn SerialPort>, SerialError>;
}

#[derive(Default)]
pub struct DefaultTransport;

impl SerialTransport for DefaultTransport {
    fn open(&self, port_name: &str, baud_rate: u32) -> Result<Box<dyn SerialPort>, SerialError> {
        serialport::new(port_name, baud_rate)
            .timeout(Duration::from_millis(50))
            .open()
            .map_err(|err| SerialError::OpenFailed(err.to_string()))
    }
}
