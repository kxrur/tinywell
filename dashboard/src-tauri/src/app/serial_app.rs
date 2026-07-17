use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::database::data::{well_data::store_data, well_reading::store_reading};
use crate::database::new_models::{NewData, NewWellReading};
use crate::serial::{ConnectionStatus, SerialManager, SerialRequest, SerialResponse};
use crate::state::AppState;
use log::{debug, info, warn};
use serde::Serialize;
use specta::Type;
use tauri::{AppHandle, Manager};
use tauri::ipc::Channel;

#[tauri::command]
#[specta::specta]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn serial_manager(state: &tauri::State<Mutex<AppState>>) -> Result<Arc<SerialManager>, String> {
    let app = state
        .lock()
        .map_err(|_| "State lock poisoned".to_string())?;
    Ok(app.serial.clone())
}

fn captured_at_ms() -> Result<i64, String> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|err| err.to_string())
        .map(|duration| duration.as_millis() as i64)
}

fn persist_environment_frame(app_handle: &AppHandle, frame: &EnvironmentFrame) -> Result<(), String> {
    let state = app_handle.state::<Mutex<AppState>>();
    let mut app = state.lock().map_err(|_| "State lock poisoned".to_string())?;
    let Some(experiment_id) = app.active_experiment_id else {
        return Ok(());
    };
    let connection = app
        .db_connection
        .as_mut()
        .ok_or_else(|| "Database is not initialized".to_string())?;

    store_data(
        connection,
        NewData {
            experiment_id,
            captured_at_ms: captured_at_ms()?,
            well_temperature_c: frame.well_temp_c as f32,
            ambient_temperature_c: frame.ambient_temp_raw as f32 / 100.0,
            ambient_pressure_pa: frame.ambient_pressure_raw as f32,
            ambient_humidity_pct: frame.ambient_humidity_raw as f32 / 1024.0,
        },
    )
    .map_err(|err| err.to_string())?;

    Ok(())
}

fn persist_sensor_frame(app_handle: &AppHandle, frame: &SensorFrame) -> Result<(), String> {
    if frame.values.len() != 14 {
        return Err(format!("Expected 14 photosensor values, received {}", frame.values.len()));
    }
    let wavelength_nm = match frame.wavelength {
        0 => 470.0,
        1 => 570.0,
        2 => 630.0,
        3 => 850.0,
        value => return Err(format!("Unknown photosensor wavelength {value}")),
    };
    let state = app_handle.state::<Mutex<AppState>>();
    let mut app = state.lock().map_err(|_| "State lock poisoned".to_string())?;
    let Some(experiment_id) = app.active_experiment_id else {
        return Ok(());
    };
    let connection = app
        .db_connection
        .as_mut()
        .ok_or_else(|| "Database is not initialized".to_string())?;
    let values = &frame.values;

    store_reading(
        connection,
        NewWellReading {
            experiment_id,
            captured_at_ms: captured_at_ms()?,
            wavelength_nm,
            well_1_intensity: values[0] as f32,
            well_2_intensity: values[1] as f32,
            well_3_intensity: values[2] as f32,
            well_4_intensity: values[3] as f32,
            well_5_intensity: values[4] as f32,
            well_6_intensity: values[5] as f32,
            well_7_intensity: values[6] as f32,
            well_8_intensity: values[7] as f32,
            well_9_intensity: values[8] as f32,
            well_10_intensity: values[9] as f32,
            well_11_intensity: values[10] as f32,
            well_12_intensity: values[11] as f32,
            well_13_intensity: values[12] as f32,
            well_14_intensity: values[13] as f32,
        },
    )
    .map_err(|err| err.to_string())?;

    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn serial_set_port(
    state: tauri::State<'_, Mutex<AppState>>,
    port: String,
) -> Result<(), String> {
    let serial = serial_manager(&state)?;
    info!("Frontend set serial port to {}", port);
    serial.set_port(port);
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn serial_connect(state: tauri::State<'_, Mutex<AppState>>) -> Result<(), String> {
    let serial = serial_manager(&state)?;
    info!("Frontend requested serial connect");
    serial.connect().map_err(|err| err.to_string())
}

#[tauri::command]
#[specta::specta]
pub fn serial_disconnect(state: tauri::State<'_, Mutex<AppState>>) -> Result<(), String> {
    let serial = serial_manager(&state)?;
    info!("Frontend requested serial disconnect");
    serial.disconnect().map_err(|err| err.to_string())
}

#[tauri::command]
#[specta::specta]
pub fn serial_status(state: tauri::State<'_, Mutex<AppState>>) -> Result<ConnectionStatus, String> {
    let serial = serial_manager(&state)?;
    Ok(serial.status())
}

#[tauri::command]
#[specta::specta]
pub fn serial_list_ports(state: tauri::State<'_, Mutex<AppState>>) -> Result<Vec<String>, String> {
    let serial = serial_manager(&state)?;
    debug!("Frontend requested port list");
    serial.list_ports().map_err(|err| err.to_string())
}

#[tauri::command]
#[specta::specta]
pub fn serial_send(
    state: tauri::State<'_, Mutex<AppState>>,
    request: SerialRequest,
    timeout_ms: Option<u32>,
) -> Result<SerialResponse, String> {
    let serial = serial_manager(&state)?;
    let timeout = Duration::from_millis(timeout_ms.unwrap_or(1500) as u64);
    debug!("Frontend sending serial request: {:?}", request);
    serial
        .send_request(request, timeout)
        .map_err(|err| err.to_string())
}

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

#[tauri::command]
#[specta::specta]
pub fn subscribe_sensor_frames(
    state: tauri::State<'_, Mutex<AppState>>,
    app_handle: AppHandle,
    channel: Channel<SensorFrame>,
) -> Result<(), String> {
    // TODO: consolidate GUI telemetry subscriptions into shared pollers before adding
    // additional channel consumers. Each subscription currently starts its own serial loop.
    let serial = serial_manager(&state)?;

    info!("Sensor frame subscription started");

    std::thread::spawn(move || loop {
        let response = serial.send_request(
            SerialRequest::PhotosensorResults,
            Duration::from_millis(1500),
        );

        match response {
            Ok(SerialResponse::PhotosensorResults { wavelength, values }) => {
                let frame = SensorFrame { values, wavelength };
                if let Err(err) = persist_sensor_frame(&app_handle, &frame) {
                    warn!("Failed to persist photosensor frame: {}", err);
                }
                if channel.send(frame).is_err() {
                    info!("Sensor frame subscription ended");
                    break;
                }
            }
            Ok(other) => {
                warn!("Sensor frame fetch unexpected response: {:?}", other);
            }
            Err(err) => {
                warn!("Sensor frame fetch failed: {}", err);
            }
        }

        std::thread::sleep(Duration::from_millis(500));
    });

    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn subscribe_environment_frames(
    state: tauri::State<'_, Mutex<AppState>>,
    app_handle: AppHandle,
    channel: Channel<EnvironmentFrame>,
) -> Result<(), String> {
    // TODO: consolidate GUI telemetry subscriptions into shared pollers before adding
    // additional channel consumers. Each subscription currently starts its own serial loop.
    let serial = serial_manager(&state)?;

    info!("Environment frame subscription started");

    std::thread::spawn(move || loop {
        let response =
            serial.send_request(SerialRequest::EnvironmentInfo, Duration::from_millis(1500));

        match response {
            Ok(SerialResponse::EnvironmentInfo {
                well_temp,
                ambient_temp,
                ambient_pressure,
                ambient_humidity,
            }) => {
                let frame = EnvironmentFrame {
                    well_temp_c: well_temp,
                    ambient_temp_raw: ambient_temp,
                    ambient_pressure_raw: ambient_pressure,
                    ambient_humidity_raw: ambient_humidity,
                };

                if let Err(err) = persist_environment_frame(&app_handle, &frame) {
                    warn!("Failed to persist environment frame: {}", err);
                }
                if channel.send(frame).is_err() {
                    info!("Environment frame subscription ended");
                    break;
                }
            }
            Ok(other) => {
                warn!("Environment frame fetch unexpected response: {:?}", other);
            }
            Err(err) => {
                warn!("Environment frame fetch failed: {}", err);
            }
        }

        std::thread::sleep(Duration::from_millis(500));
    });

    Ok(())
}
