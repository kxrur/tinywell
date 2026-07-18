use std::path::Path;

use csv::Writer;

use super::service::{
    experiment_file_stem, format_local_timestamp, ExperimentExport, ExperimentExporter,
};

pub struct CsvExporter;

impl ExperimentExporter for CsvExporter {
    const FILES_PER_EXPERIMENT: u32 = 2;

    fn export_experiment(
        &self,
        directory: &Path,
        experiment: &ExperimentExport,
    ) -> Result<(), String> {
        let stem = experiment_file_stem(experiment)?;
        write_environment_csv(
            &directory.join(format!("{stem}_environment.csv")),
            experiment,
        )?;
        write_well_readings_csv(
            &directory.join(format!("{stem}_well-readings.csv")),
            experiment,
        )
    }
}

fn write_environment_csv(path: &Path, export: &ExperimentExport) -> Result<(), String> {
    let mut writer = Writer::from_path(path).map_err(|err| err.to_string())?;
    writer
        .write_record([
            "captured_at",
            "well_temperature_c",
            "ambient_temperature_c",
            "ambient_pressure_pa",
            "ambient_humidity_pct",
        ])
        .map_err(|err| err.to_string())?;
    for row in &export.environment {
        writer
            .write_record([
                format_local_timestamp(row.captured_at_ms),
                row.well_temperature_c.to_string(),
                row.ambient_temperature_c.to_string(),
                row.ambient_pressure_pa.to_string(),
                row.ambient_humidity_pct.to_string(),
            ])
            .map_err(|err| err.to_string())?;
    }
    writer.flush().map_err(|err| err.to_string())
}

fn write_well_readings_csv(path: &Path, export: &ExperimentExport) -> Result<(), String> {
    let mut writer = Writer::from_path(path).map_err(|err| err.to_string())?;
    writer
        .write_record([
            "captured_at",
            "wavelength_nm",
            "well_1_intensity",
            "well_2_intensity",
            "well_3_intensity",
            "well_4_intensity",
            "well_5_intensity",
            "well_6_intensity",
            "well_7_intensity",
            "well_8_intensity",
            "well_9_intensity",
            "well_10_intensity",
            "well_11_intensity",
            "well_12_intensity",
            "well_13_intensity",
            "well_14_intensity",
        ])
        .map_err(|err| err.to_string())?;
    for row in &export.readings {
        writer
            .write_record([
                format_local_timestamp(row.captured_at_ms),
                row.wavelength_nm.to_string(),
                row.well_1_intensity.to_string(),
                row.well_2_intensity.to_string(),
                row.well_3_intensity.to_string(),
                row.well_4_intensity.to_string(),
                row.well_5_intensity.to_string(),
                row.well_6_intensity.to_string(),
                row.well_7_intensity.to_string(),
                row.well_8_intensity.to_string(),
                row.well_9_intensity.to_string(),
                row.well_10_intensity.to_string(),
                row.well_11_intensity.to_string(),
                row.well_12_intensity.to_string(),
                row.well_13_intensity.to_string(),
                row.well_14_intensity.to_string(),
            ])
            .map_err(|err| err.to_string())?;
    }
    writer.flush().map_err(|err| err.to_string())
}
