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

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct SystemStatusFrame {
    pub firmware_version: u8,
    pub state: u8,
    pub uptime_seconds: u32,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ExperimentStatusFrame {
    pub last_state: u8,
    pub progress: u8,
    pub elapsed_or_started_at_uptime: u32,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct HardwareHealthFrame {
    pub bipump_current: u16,
    pub heater_current: u16,
    pub power_supply_sense: u16,
}

#[derive(Clone, Debug, Default, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentFrame {
    pub system_status: Option<SystemStatusFrame>,
    pub experiment_status: Option<ExperimentStatusFrame>,
    pub hardware_health: Option<HardwareHealthFrame>,
    pub last_error: Option<String>,
}
