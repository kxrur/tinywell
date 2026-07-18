use std::path::Path;

use rust_xlsxwriter::{Format, Table, TableColumn, Workbook, Worksheet};

use super::service::{experiment_file_stem, local_datetime, ExperimentExport, ExperimentExporter};

pub struct ExcelExporter;

impl ExperimentExporter for ExcelExporter {
    const FILES_PER_EXPERIMENT: u32 = 1;

    fn export_experiment(
        &self,
        directory: &Path,
        experiment: &ExperimentExport,
    ) -> Result<(), String> {
        let path = directory.join(format!("{}.xlsx", experiment_file_stem(experiment)?));
        let mut workbook = Workbook::new();
        let worksheet = workbook
            .add_worksheet()
            .set_name("Data")
            .map_err(|err| err.to_string())?;
        let timestamp_format = Format::new().set_num_format("yyyy-mm-dd hh:mm:ss.000");

        worksheet
            .write_string(0, 0, "Environment")
            .map_err(|err| err.to_string())?;
        write_environment_table(worksheet, 1, experiment, &timestamp_format)?;

        const WELL_READINGS_START_COLUMN: u16 = 7;
        worksheet
            .write_string(0, WELL_READINGS_START_COLUMN, "Well Readings")
            .map_err(|err| err.to_string())?;
        write_well_readings_table(
            worksheet,
            1,
            WELL_READINGS_START_COLUMN,
            experiment,
            &timestamp_format,
        )?;
        worksheet
            .set_column_width(0, 24)
            .map_err(|err| err.to_string())?;
        worksheet
            .set_column_range_width(1, 4, 18)
            .map_err(|err| err.to_string())?;
        worksheet
            .set_column_range_width(7, 22, 18)
            .map_err(|err| err.to_string())?;
        workbook.save(path).map_err(|err| err.to_string())
    }
}

fn write_environment_table(
    worksheet: &mut Worksheet,
    header_row: u32,
    export: &ExperimentExport,
    timestamp_format: &Format,
) -> Result<(), String> {
    for (index, row) in export.environment.iter().enumerate() {
        let output_row = header_row
            + 1
            + u32::try_from(index).map_err(|_| "Too many environment rows".to_string())?;
        write_timestamp(
            worksheet,
            output_row,
            0,
            row.captured_at_ms,
            timestamp_format,
        )?;
        worksheet
            .write_number(output_row, 1, row.well_temperature_c)
            .map_err(|err| err.to_string())?;
        worksheet
            .write_number(output_row, 2, row.ambient_temperature_c)
            .map_err(|err| err.to_string())?;
        worksheet
            .write_number(output_row, 3, row.ambient_pressure_pa)
            .map_err(|err| err.to_string())?;
        worksheet
            .write_number(output_row, 4, row.ambient_humidity_pct)
            .map_err(|err| err.to_string())?;
    }
    add_table(
        worksheet,
        header_row,
        0,
        export.environment.len(),
        4,
        "Environment",
        &[
            "captured_at",
            "well_temperature_c",
            "ambient_temperature_c",
            "ambient_pressure_pa",
            "ambient_humidity_pct",
        ],
    )
}

fn write_well_readings_table(
    worksheet: &mut Worksheet,
    header_row: u32,
    start_column: u16,
    export: &ExperimentExport,
    timestamp_format: &Format,
) -> Result<(), String> {
    for (index, row) in export.readings.iter().enumerate() {
        let output_row = header_row
            + 1
            + u32::try_from(index).map_err(|_| "Too many well-reading rows".to_string())?;
        write_timestamp(
            worksheet,
            output_row,
            start_column,
            row.captured_at_ms,
            timestamp_format,
        )?;
        let values = [
            row.wavelength_nm,
            row.well_1_intensity,
            row.well_2_intensity,
            row.well_3_intensity,
            row.well_4_intensity,
            row.well_5_intensity,
            row.well_6_intensity,
            row.well_7_intensity,
            row.well_8_intensity,
            row.well_9_intensity,
            row.well_10_intensity,
            row.well_11_intensity,
            row.well_12_intensity,
            row.well_13_intensity,
            row.well_14_intensity,
        ];
        for (column, value) in values.into_iter().enumerate() {
            worksheet
                .write_number(
                    output_row,
                    start_column
                        + u16::try_from(column + 1).map_err(|_| "Too many columns".to_string())?,
                    value,
                )
                .map_err(|err| err.to_string())?;
        }
    }
    add_table(
        worksheet,
        header_row,
        start_column,
        export.readings.len(),
        start_column + 15,
        "WellReadings",
        &[
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
        ],
    )
}

fn add_table(
    worksheet: &mut Worksheet,
    header_row: u32,
    start_column: u16,
    row_count: usize,
    end_column: u16,
    name: &str,
    headers: &[&str],
) -> Result<(), String> {
    let last_row = header_row
        + u32::try_from(row_count.max(1)).map_err(|_| "Too many rows to export".to_string())?;
    let columns: Vec<_> = headers
        .iter()
        .map(|header| TableColumn::new().set_header(*header))
        .collect();
    worksheet
        .add_table(
            header_row,
            start_column,
            last_row,
            end_column,
            &Table::new().set_name(name).set_columns(&columns),
        )
        .map_err(|err| err.to_string())?;
    Ok(())
}

fn write_timestamp(
    worksheet: &mut Worksheet,
    row: u32,
    column: u16,
    captured_at_ms: i64,
    timestamp_format: &Format,
) -> Result<(), String> {
    match local_datetime(captured_at_ms) {
        Some(timestamp) => worksheet
            .write_datetime_with_format(row, column, timestamp, timestamp_format)
            .map_err(|err| err.to_string())?,
        None => worksheet
            .write_string(row, column, captured_at_ms.to_string())
            .map_err(|err| err.to_string())?,
    };
    Ok(())
}
