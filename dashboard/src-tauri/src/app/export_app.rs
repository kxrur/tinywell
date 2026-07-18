use std::sync::Mutex;

use crate::export::{
    csv::CsvExporter,
    excel::ExcelExporter,
    service::{export_experiments, ExportSummary},
};
use crate::state::AppState;

#[tauri::command]
#[specta::specta]
pub fn export_csv_experiments(
    state: tauri::State<'_, Mutex<AppState>>,
    output_directory: String,
    experiment_ids: Option<Vec<i32>>,
) -> Result<ExportSummary, String> {
    export_experiments(&state, output_directory, experiment_ids, CsvExporter)
}

#[tauri::command]
#[specta::specta]
pub fn export_excel_experiments(
    state: tauri::State<'_, Mutex<AppState>>,
    output_directory: String,
    experiment_ids: Option<Vec<i32>>,
) -> Result<ExportSummary, String> {
    export_experiments(&state, output_directory, experiment_ids, ExcelExporter)
}
