#[cfg(test)]
mod tests {
    use crate::serial::{Bme280, SerialSensor};
    use std::io::{BufRead, BufReader};
    use std::time::Duration;

    #[test]
    #[ignore = "Requires hardware"]
    fn test_bme280_read_live_data() {
        let port_name = "/dev/ttyACM0"; // Change to your specific hardcoded port
        let mut bme = Bme280::new();

        println!("Attempting to connect to {}", port_name);

        let port_result = serialport::new(port_name, 9600)
            .timeout(Duration::from_secs(5))
            .open();

        match port_result {
            Ok(port) => {
                println!("Connected! Reading data for 5 seconds...");
                let mut reader = BufReader::new(port);
                let start_time = std::time::Instant::now();

                // Read for 5 seconds
                while start_time.elapsed() < Duration::from_secs(5) {
                    let mut line = String::new();
                    // Read a line from the serial buffer
                    if reader.read_line(&mut line).is_ok() && !line.is_empty() {
                        let line = line.trim();
                        println!("Raw RX: {}", line);
                        if bme.parse_line(line) {
                            println!("Parsed State: {:?}", bme.data);
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to open port {}: {}", port_name, e);
            }
        }
    }

    #[test]
    fn test_bme280_parse_string() {
        let mut bme = Bme280::new();

        assert!(bme.parse_line("Temperature[0]:   26.5 deg C"));
        assert_eq!(bme.data.temperature, 26.5);

        assert!(bme.parse_line("Humidity[0]:   45.2 %RH"));
        assert_eq!(bme.data.humidity, 45.2);

        assert!(bme.parse_line("Pressure[0]:   101325 Pa"));
        assert_eq!(bme.data.pressure, 101325.0);
    }
}
