// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use std::sync::Mutex;

use log::info;
use specta_typescript::Typescript;
use tauri::Manager;
use tauri_specta::*;

mod app;
mod database;
mod export;
mod model;
pub mod serial;
mod state;
mod telemetry;

use crate::app::experiment_app::{
    experiment_create, experiment_delete, experiment_list, experiment_set_active,
};
use crate::app::export_app::{export_csv_experiments, export_excel_experiments};
use crate::app::history_app::history_load_experiment;
use crate::app::serial_app::{
    greet, serial_connect, serial_disconnect, serial_list_ports, serial_send, serial_set_port,
    serial_status, subscribe_environment_frames, subscribe_sensor_frames,
};
use crate::database::sqlite::init_database;
use crate::state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let _ = env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug"))
        .try_init();
    info!("Tinywell backend starting");

    let builder = Builder::<tauri::Wry>::new()
        .commands(collect_commands![
            greet,
            experiment_create,
            experiment_delete,
            experiment_list,
            experiment_set_active,
            export_csv_experiments,
            export_excel_experiments,
            history_load_experiment,
            serial_set_port,
            serial_connect,
            serial_disconnect,
            serial_status,
            serial_list_ports,
            serial_send,
            subscribe_sensor_frames,
            subscribe_environment_frames
        ])
        .events(collect_events![]);

    #[cfg(debug_assertions)] // <- Only export on non-release builds
    builder
        .export(Typescript::default(), "../src/bindings.ts")
        .expect("Failed to export typescript bindings");

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(builder.invoke_handler())
        .setup(move |app| {
            app.manage(Mutex::new(AppState::default()));

            builder.mount_events(app);

            init_database(app.app_handle())?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
