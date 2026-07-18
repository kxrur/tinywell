use std::path::{Path, PathBuf};
use std::sync::Mutex;

use chrono::{Local, NaiveDateTime, TimeZone};
use serde::Serialize;
use specta::Type;

use crate::database::data::{
    experiment::list_experiments, well_data::list_data_for_experiment,
    well_reading::list_readings_for_experiment,
};
use crate::database::models::{Data, Experiment, WellReading};
use crate::state::AppState;

#[derive(Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ExportSummary {
    pub experiments_exported: u32,
    pub files_written: u32,
}

pub(crate) trait ExperimentExporter {
    const FILES_PER_EXPERIMENT: u32;

    fn export_experiment(
        &self,
        directory: &Path,
        experiment: &ExperimentExport,
    ) -> Result<(), String>;
}

pub fn export_experiments<E: ExperimentExporter>(
    state: &tauri::State<'_, Mutex<AppState>>,
    output_directory: String,
    experiment_ids: Option<Vec<i32>>,
    exporter: E,
) -> Result<ExportSummary, String> {
    let directory = export_directory(output_directory)?;
    let exports = load_exports(state, experiment_ids)?;

    for experiment in &exports {
        exporter.export_experiment(&directory, experiment)?;
    }

    Ok(ExportSummary {
        experiments_exported: u32::try_from(exports.len())
            .map_err(|_| "Too many experiments to export".to_string())?,
        files_written: u32::try_from(exports.len())
            .map_err(|_| "Too many experiments to export".to_string())?
            .checked_mul(E::FILES_PER_EXPERIMENT)
            .ok_or_else(|| "Too many files to export".to_string())?,
    })
}

pub(crate) struct ExperimentExport {
    pub(super) experiment: Experiment,
    pub(super) environment: Vec<Data>,
    pub(super) readings: Vec<WellReading>,
}

pub(super) fn experiment_file_stem(export: &ExperimentExport) -> Result<String, String> {
    let id = export
        .experiment
        .id
        .ok_or_else(|| "Experiment has no ID".to_string())?;
    Ok(format!(
        "{}_{}",
        safe_file_name(&export.experiment.name),
        id
    ))
}

pub(super) fn format_local_timestamp(captured_at_ms: i64) -> String {
    local_datetime(captured_at_ms)
        .map(|timestamp| timestamp.format("%Y-%m-%d %H:%M:%S%.3f").to_string())
        .unwrap_or_else(|| captured_at_ms.to_string())
}

pub(super) fn local_datetime(captured_at_ms: i64) -> Option<NaiveDateTime> {
    Local
        .timestamp_millis_opt(captured_at_ms)
        .single()
        .map(|timestamp| timestamp.naive_local())
}

fn export_directory(output_directory: String) -> Result<PathBuf, String> {
    let directory = PathBuf::from(output_directory);
    directory
        .is_dir()
        .then_some(directory)
        .ok_or_else(|| "Export location must be an existing folder".to_string())
}

fn load_exports(
    state: &tauri::State<'_, Mutex<AppState>>,
    requested_ids: Option<Vec<i32>>,
) -> Result<Vec<ExperimentExport>, String> {
    let mut app = state
        .lock()
        .map_err(|_| "State lock poisoned".to_string())?;
    let experiments = list_experiments(app.db_connection_mut()?).map_err(|err| err.to_string())?;
    let selected = match requested_ids {
        Some(ids) => {
            let selected: Vec<_> = experiments
                .into_iter()
                .filter(|experiment| experiment.id.is_some_and(|id| ids.contains(&id)))
                .collect();
            if selected.len() != ids.len() {
                return Err("One or more requested experiments no longer exist".to_string());
            }
            selected
        }
        None => experiments,
    };

    selected
        .into_iter()
        .map(|experiment| {
            let id = experiment
                .id
                .ok_or_else(|| "Experiment has no ID".to_string())?;
            let environment = list_data_for_experiment(app.db_connection_mut()?, id)
                .map_err(|err| err.to_string())?;
            let readings = list_readings_for_experiment(app.db_connection_mut()?, id)
                .map_err(|err| err.to_string())?;
            Ok(ExperimentExport {
                experiment,
                environment,
                readings,
            })
        })
        .collect()
}

fn safe_file_name(name: &str) -> String {
    let value: String = name
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || matches!(character, '-' | '_') {
                character
            } else {
                '_'
            }
        })
        .collect();
    let value = value.trim_matches('_');
    if value.is_empty() {
        "experiment".to_string()
    } else {
        value.to_string()
    }
}
