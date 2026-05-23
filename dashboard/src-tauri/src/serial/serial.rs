use serialport::{available_ports, SerialPortInfo, SerialPortType};

pub struct Serial {
    ports: Vec<SerialPortInfo>,
    port_selected: usize,
}

impl Default for Serial {
    fn default() -> Self {
        let ports = available_ports().expect("Couldn't retrieve available ports.");

        Serial {
            ports,
            port_selected: 0,
        }
    }
}

impl Serial {
    pub fn get_port_name(port: &SerialPortInfo) -> String {
        match &port.port_type {
            SerialPortType::UsbPort(info) => {
                if info.vid == 0x16C0 {
                    String::from("Teensyduino")
                } else {
                    port.clone().port_name
                }
            }
            _ => port.clone().port_name,
        }
    }

    pub fn get_ports(&mut self) -> serialport::Result<Vec<SerialPortInfo>> {
        self.ports = available_ports()?;
        Ok(self.ports.clone())
    }

    pub fn get_active_port_name(&self) -> Option<String> {
        self.ports
            .get(self.port_selected)
            .map(|info| info.port_name.clone())
    }
}
