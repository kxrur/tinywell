use serde::Serialize;
use specta::Type;

#[derive(Clone, Debug, Serialize, Type)]
pub struct SensorFrame {
    pub values: Vec<u32>,
    pub wavelength: u8,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct EnvironmentFrame {
    pub well_temp_c: u16,
    pub ambient_temp_raw: i32,
    pub ambient_pressure_raw: u32,
    pub ambient_humidity_raw: u32,
}
