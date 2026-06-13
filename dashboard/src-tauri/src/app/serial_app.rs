use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::serial::{ConnectionStatus, SerialManager, SerialRequest, SerialResponse};
use crate::state::AppState;
use log::{debug, info, warn};
use serde::Serialize;
use specta::Type;
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
    channel: Channel<SensorFrame>,
) -> Result<(), String> {
    let serial = serial_manager(&state)?;

    info!("Sensor frame subscription started");

    std::thread::spawn(move || loop {
        let response = serial.send_request(
            SerialRequest::PhotosensorResults,
            Duration::from_millis(1500),
        );

        match response {
            Ok(SerialResponse::PhotosensorResults { wavelength, values }) => {
                if channel.send(SensorFrame { values, wavelength }).is_err() {
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
    channel: Channel<EnvironmentFrame>,
) -> Result<(), String> {
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
