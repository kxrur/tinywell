use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::serial::{ConnectionStatus, SerialManager, SerialRequest, SerialResponse};
use crate::state::AppState;
use crate::telemetry::{EnvironmentFrame, SensorFrame};
use log::{debug, info};
use tauri::ipc::Channel;
use tauri::AppHandle;

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

#[tauri::command]
#[specta::specta]
pub fn subscribe_sensor_frames(
    state: tauri::State<'_, Mutex<AppState>>,
    app_handle: AppHandle,
    channel: Channel<SensorFrame>,
) -> Result<(), String> {
    let serial = serial_manager(&state)?;
    let telemetry = {
        let app = state
            .lock()
            .map_err(|_| "State lock poisoned".to_string())?;
        app.telemetry.clone()
    };
    telemetry.subscribe_sensor(channel, serial, app_handle);

    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn subscribe_environment_frames(
    state: tauri::State<'_, Mutex<AppState>>,
    app_handle: AppHandle,
    channel: Channel<EnvironmentFrame>,
) -> Result<(), String> {
    let serial = serial_manager(&state)?;
    let telemetry = {
        let app = state
            .lock()
            .map_err(|_| "State lock poisoned".to_string())?;
        app.telemetry.clone()
    };
    telemetry.subscribe_environment(channel, serial, app_handle);

    Ok(())
}
