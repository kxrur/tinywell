use super::traits::SerialSensor;
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, Clone, Default, Serialize, Deserialize, Type)]
pub struct Bme280Data {
    pub temperature: f32, // Celsius
    pub humidity: f32,    // %RH
    pub pressure: f32,    // Pascals
}

#[derive(Debug, Default, Clone)]
pub struct Bme280 {
    pub data: Bme280Data,
}

impl Bme280 {
    pub fn new() -> Self {
        Self::default()
    }

    /// Extracts a numeric value from the Arduino's formatted output.
    fn extract_value(line: &str) -> Option<f32> {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() == 2 {
            // "   25.5 deg C" -> "25.5"
            let str_val = parts[1].trim().split_whitespace().next()?;
            str_val.parse::<f32>().ok()
        } else {
            None
        }
    }
}

impl SerialSensor for Bme280 {
    fn name(&self) -> &str {
        "BME280"
    }

    fn parse_line(&mut self, line: &str) -> bool {
        let mut updated = false;

        if line.starts_with("Temperature") {
            if let Some(val) = Self::extract_value(line) {
                self.data.temperature = val;
                updated = true;
            }
        } else if line.starts_with("Humidity") {
            if let Some(val) = Self::extract_value(line) {
                self.data.humidity = val;
                updated = true;
            }
        } else if line.starts_with("Pressure") {
            if let Some(val) = Self::extract_value(line) {
                self.data.pressure = val;
                updated = true;
            }
        }

        updated
    }
}
