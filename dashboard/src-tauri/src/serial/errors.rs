use std::fmt;

#[derive(Debug, Clone)]
pub enum SerialError {
    PortNotSet,
    NotConnected,
    OpenFailed(String),
    IoError(String),
    Timeout,
    Protocol(String),
    DeviceError(u8),
    ChannelClosed,
}

impl fmt::Display for SerialError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SerialError::PortNotSet => write!(f, "Serial port not set"),
            SerialError::NotConnected => write!(f, "Serial port not connected"),
            SerialError::OpenFailed(msg) => write!(f, "Failed to open serial port: {}", msg),
            SerialError::IoError(msg) => write!(f, "Serial IO error: {}", msg),
            SerialError::Timeout => write!(f, "Serial request timed out"),
            SerialError::Protocol(msg) => write!(f, "Serial protocol error: {}", msg),
            SerialError::DeviceError(code) => {
                write!(f, "Device returned error code 0x{:02X}", code)
            }
            SerialError::ChannelClosed => write!(f, "Serial request channel closed"),
        }
    }
}

impl std::error::Error for SerialError {}
