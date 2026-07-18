use std::sync::Mutex;

use serde::Serialize;
use specta::Type;

use crate::database::data::{
    experiment::experiment_exists, well_data::list_recent_data_for_experiment,
    well_reading::list_recent_readings_for_experiment,
};
use crate::database::models::{Data, WellReading};
use crate::state::AppState;

#[derive(Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct HistorySnapshot {
    pub environment: Vec<EnvironmentHistoryRow>,
    pub readings: Vec<WellReadingHistoryRow>,
}

#[derive(Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct EnvironmentHistoryRow {
    pub captured_at_ms: f64,
    pub well_temperature_c: f32,
    pub ambient_temperature_c: f32,
    pub ambient_pressure_pa: f32,
    pub ambient_humidity_pct: f32,
}

impl From<Data> for EnvironmentHistoryRow {
    fn from(row: Data) -> Self {
        Self {
            captured_at_ms: row.captured_at_ms as f64,
            well_temperature_c: row.well_temperature_c,
            ambient_temperature_c: row.ambient_temperature_c,
            ambient_pressure_pa: row.ambient_pressure_pa,
            ambient_humidity_pct: row.ambient_humidity_pct,
        }
    }
}

#[derive(Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct WellReadingHistoryRow {
    pub captured_at_ms: f64,
    pub wavelength_nm: f32,
    pub well_1_intensity: f32,
    pub well_2_intensity: f32,
    pub well_3_intensity: f32,
    pub well_4_intensity: f32,
    pub well_5_intensity: f32,
    pub well_6_intensity: f32,
    pub well_7_intensity: f32,
    pub well_8_intensity: f32,
    pub well_9_intensity: f32,
    pub well_10_intensity: f32,
    pub well_11_intensity: f32,
    pub well_12_intensity: f32,
    pub well_13_intensity: f32,
    pub well_14_intensity: f32,
}

impl From<WellReading> for WellReadingHistoryRow {
    fn from(row: WellReading) -> Self {
        Self {
            captured_at_ms: row.captured_at_ms as f64,
            wavelength_nm: row.wavelength_nm,
            well_1_intensity: row.well_1_intensity,
            well_2_intensity: row.well_2_intensity,
            well_3_intensity: row.well_3_intensity,
            well_4_intensity: row.well_4_intensity,
            well_5_intensity: row.well_5_intensity,
            well_6_intensity: row.well_6_intensity,
            well_7_intensity: row.well_7_intensity,
            well_8_intensity: row.well_8_intensity,
            well_9_intensity: row.well_9_intensity,
            well_10_intensity: row.well_10_intensity,
            well_11_intensity: row.well_11_intensity,
            well_12_intensity: row.well_12_intensity,
            well_13_intensity: row.well_13_intensity,
            well_14_intensity: row.well_14_intensity,
        }
    }
}

#[tauri::command]
#[specta::specta]
pub fn history_load_experiment(
    state: tauri::State<'_, Mutex<AppState>>,
    experiment_id: i32,
    max_rows: u32,
) -> Result<HistorySnapshot, String> {
    let mut app = state
        .lock()
        .map_err(|_| "State lock poisoned".to_string())?;
    if !experiment_exists(app.db_connection_mut()?, experiment_id).map_err(|err| err.to_string())? {
        return Err(format!("Experiment {experiment_id} does not exist"));
    }

    let max_rows = i64::from(max_rows);
    let environment =
        list_recent_data_for_experiment(app.db_connection_mut()?, experiment_id, max_rows)
            .map_err(|err| err.to_string())?;
    let readings =
        list_recent_readings_for_experiment(app.db_connection_mut()?, experiment_id, max_rows)
            .map_err(|err| err.to_string())?;

    Ok(HistorySnapshot {
        environment: environment.into_iter().map(Into::into).collect(),
        readings: readings.into_iter().map(Into::into).collect(),
    })
}
