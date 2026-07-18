use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::database::data::{well_data::store_data, well_reading::store_reading};
use crate::database::new_models::{NewData, NewWellReading};
use crate::serial::{SerialManager, SerialRequest, SerialResponse};
use crate::state::AppState;
use crate::telemetry::{EnvironmentFrame, SensorFrame};
use log::{info, warn};
use tauri::ipc::Channel;
use tauri::{AppHandle, Manager};

#[derive(Default)]
pub struct TelemetryManager {
    sensor: Mutex<TelemetrySubscribers<SensorFrame>>,
    environment: Mutex<TelemetrySubscribers<EnvironmentFrame>>,
}

struct TelemetrySubscribers<T> {
    channels: Vec<Channel<T>>,
    worker_started: bool,
    generation: u64,
}

impl<T> Default for TelemetrySubscribers<T> {
    fn default() -> Self {
        Self {
            channels: Vec::new(),
            worker_started: false,
            generation: 0,
        }
    }
}

impl TelemetryManager {
    pub fn subscribe_sensor(
        self: &Arc<Self>,
        channel: Channel<SensorFrame>,
        serial: Arc<SerialManager>,
        app_handle: AppHandle,
    ) {
        let generation = {
            let mut subscribers = self.sensor.lock().expect("sensor telemetry lock");
            subscribers.channels.push(channel);
            if subscribers.worker_started {
                None
            } else {
                subscribers.worker_started = true;
                subscribers.generation += 1;
                Some(subscribers.generation)
            }
        };

        if let Some(generation) = generation {
            let telemetry = Arc::clone(self);
            std::thread::spawn(move || sensor_worker(serial, app_handle, telemetry, generation));
        }
    }

    pub fn subscribe_environment(
        self: &Arc<Self>,
        channel: Channel<EnvironmentFrame>,
        serial: Arc<SerialManager>,
        app_handle: AppHandle,
    ) {
        let generation = {
            let mut subscribers = self.environment.lock().expect("environment telemetry lock");
            subscribers.channels.push(channel);
            if subscribers.worker_started {
                None
            } else {
                subscribers.worker_started = true;
                subscribers.generation += 1;
                Some(subscribers.generation)
            }
        };

        if let Some(generation) = generation {
            let telemetry = Arc::clone(self);
            std::thread::spawn(move || {
                environment_worker(serial, app_handle, telemetry, generation)
            });
        }
    }

    pub fn stop(&self) {
        stop_subscribers(&self.sensor, "photosensor");
        stop_subscribers(&self.environment, "environment");
    }

    fn sensor_worker_active(&self, generation: u64) -> bool {
        worker_active(&self.sensor, generation)
    }

    fn environment_worker_active(&self, generation: u64) -> bool {
        worker_active(&self.environment, generation)
    }

    fn broadcast_sensor(&self, frame: &SensorFrame) {
        let mut subscribers = self.sensor.lock().expect("sensor telemetry lock");
        subscribers
            .channels
            .retain(|channel| channel.send(frame.clone()).is_ok());
    }

    fn broadcast_environment(&self, frame: &EnvironmentFrame) {
        let mut subscribers = self.environment.lock().expect("environment telemetry lock");
        subscribers
            .channels
            .retain(|channel| channel.send(frame.clone()).is_ok());
    }
}

fn stop_subscribers<T>(subscribers: &Mutex<TelemetrySubscribers<T>>, name: &str) {
    let mut subscribers = subscribers.lock().expect("telemetry lock");
    subscribers.channels.clear();
    subscribers.worker_started = false;
    subscribers.generation += 1;
    info!("Shared {} telemetry worker stopped", name);
}

fn worker_active<T>(subscribers: &Mutex<TelemetrySubscribers<T>>, generation: u64) -> bool {
    let subscribers = subscribers.lock().expect("telemetry lock");
    subscribers.worker_started && subscribers.generation == generation
}

fn sensor_worker(
    serial: Arc<SerialManager>,
    app_handle: AppHandle,
    telemetry: Arc<TelemetryManager>,
    generation: u64,
) {
    info!("Shared photosensor telemetry worker started");

    while telemetry.sensor_worker_active(generation) {
        match serial.send_request(
            SerialRequest::PhotosensorResults,
            Duration::from_millis(1500),
        ) {
            Ok(SerialResponse::PhotosensorResults { wavelength, values }) => {
                if !telemetry.sensor_worker_active(generation) {
                    break;
                }
                let frame = SensorFrame { values, wavelength };
                if let Err(err) = persist_sensor_frame(&app_handle, &frame) {
                    warn!("Failed to persist photosensor frame: {}", err);
                }
                telemetry.broadcast_sensor(&frame);
            }
            Ok(other) => warn!("Sensor frame fetch unexpected response: {:?}", other),
            Err(err) => warn!("Sensor frame fetch failed: {}", err),
        }

        std::thread::sleep(Duration::from_millis(500));
    }

    info!("Shared photosensor telemetry worker exited");
}

fn environment_worker(
    serial: Arc<SerialManager>,
    app_handle: AppHandle,
    telemetry: Arc<TelemetryManager>,
    generation: u64,
) {
    info!("Shared environment telemetry worker started");

    while telemetry.environment_worker_active(generation) {
        match serial.send_request(SerialRequest::EnvironmentInfo, Duration::from_millis(1500)) {
            Ok(SerialResponse::EnvironmentInfo {
                well_temp,
                ambient_temp,
                ambient_pressure,
                ambient_humidity,
            }) => {
                if !telemetry.environment_worker_active(generation) {
                    break;
                }
                let frame = EnvironmentFrame {
                    well_temp_c: well_temp,
                    ambient_temp_raw: ambient_temp,
                    ambient_pressure_raw: ambient_pressure,
                    ambient_humidity_raw: ambient_humidity,
                };
                if let Err(err) = persist_environment_frame(&app_handle, &frame) {
                    warn!("Failed to persist environment frame: {}", err);
                }
                telemetry.broadcast_environment(&frame);
            }
            Ok(other) => warn!("Environment frame fetch unexpected response: {:?}", other),
            Err(err) => warn!("Environment frame fetch failed: {}", err),
        }

        std::thread::sleep(Duration::from_millis(500));
    }

    info!("Shared environment telemetry worker exited");
}

fn persist_environment_frame(
    app_handle: &AppHandle,
    frame: &EnvironmentFrame,
) -> Result<(), String> {
    let state = app_handle.state::<Mutex<AppState>>();
    let mut app = state
        .lock()
        .map_err(|_| "State lock poisoned".to_string())?;
    let Some(experiment_id) = app.active_experiment_id else {
        return Ok(());
    };

    store_data(
        app.db_connection_mut()?,
        NewData {
            experiment_id,
            captured_at_ms: captured_at_ms()?,
            well_temperature_c: frame.well_temp_c as f32,
            ambient_temperature_c: frame.ambient_temp_raw as f32 / 100.0,
            ambient_pressure_pa: frame.ambient_pressure_raw as f32,
            ambient_humidity_pct: frame.ambient_humidity_raw as f32 / 1024.0,
        },
    )
    .map(|_| ())
    .map_err(|err| err.to_string())
}

fn persist_sensor_frame(app_handle: &AppHandle, frame: &SensorFrame) -> Result<(), String> {
    if frame.values.len() != 14 {
        return Err(format!(
            "Expected 14 photosensor values, received {}",
            frame.values.len()
        ));
    }
    let wavelength_nm = match frame.wavelength {
        0 => 470.0,
        1 => 570.0,
        2 => 630.0,
        3 => 850.0,
        value => return Err(format!("Unknown photosensor wavelength {value}")),
    };

    let state = app_handle.state::<Mutex<AppState>>();
    let mut app = state
        .lock()
        .map_err(|_| "State lock poisoned".to_string())?;
    let Some(experiment_id) = app.active_experiment_id else {
        return Ok(());
    };
    let values = &frame.values;

    store_reading(
        app.db_connection_mut()?,
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
    .map(|_| ())
    .map_err(|err| err.to_string())
}

fn captured_at_ms() -> Result<i64, String> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|err| err.to_string())
        .map(|duration| duration.as_millis() as i64)
}
