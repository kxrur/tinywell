use std::sync::Mutex;

use crate::database::data::experiment::{
    experiment_exists, experiment_name_exists, list_experiments, store_experiment,
};
use crate::database::models::Experiment;
use crate::database::new_models::NewExperiment;
use crate::state::AppState;

#[tauri::command]
#[specta::specta]
pub fn experiment_create(
    state: tauri::State<'_, Mutex<AppState>>,
    name: String,
) -> Result<Experiment, String> {
    let name = name.trim();
    if name.is_empty() {
        return Err("Experiment name cannot be empty".to_string());
    }

    let mut app = state.lock().map_err(|_| "State lock poisoned".to_string())?;
    if experiment_name_exists(app.db_connection_mut()?, name).map_err(|err| err.to_string())? {
        return Err(format!("An experiment named '{name}' already exists"));
    }
    store_experiment(app.db_connection_mut()?, NewExperiment { name: name.into() })
        .map_err(|err| err.to_string())?;

    list_experiments(app.db_connection_mut()?)
        .map_err(|err| err.to_string())?
        .into_iter()
        .next()
        .ok_or_else(|| "Created experiment could not be loaded".to_string())
}

#[tauri::command]
#[specta::specta]
pub fn experiment_list(
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<Vec<Experiment>, String> {
    let mut app = state.lock().map_err(|_| "State lock poisoned".to_string())?;
    list_experiments(app.db_connection_mut()?)
        .map_err(|err| err.to_string())
}

#[tauri::command]
#[specta::specta]
pub fn experiment_set_active(
    state: tauri::State<'_, Mutex<AppState>>,
    experiment_id: i32,
) -> Result<(), String> {
    let mut app = state.lock().map_err(|_| "State lock poisoned".to_string())?;
    if !experiment_exists(app.db_connection_mut()?, experiment_id).map_err(|err| err.to_string())? {
        return Err(format!("Experiment {experiment_id} does not exist"));
    }
    app.active_experiment_id = Some(experiment_id);
    Ok(())
}
