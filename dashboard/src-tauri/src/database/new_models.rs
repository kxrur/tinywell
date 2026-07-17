#![allow(unused)]
#![allow(clippy::all)]

use diesel::prelude::*;

#[derive(Insertable)]
#[diesel(table_name = crate::database::schema::experiments)]
pub struct NewExperiment {
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::database::schema::data)]
pub struct NewData {
    pub experiment_id: i32,
    pub captured_at_ms: i64,
    pub well_temperature_c: f32,
    pub ambient_temperature_c: f32,
    pub ambient_pressure_pa: f32,
    pub ambient_humidity_pct: f32,
}

#[derive(Insertable)]
#[diesel(table_name = crate::database::schema::well_readings)]
pub struct NewWellReading {
    pub experiment_id: i32,
    pub captured_at_ms: i64,
    pub wavelength_nm: f32,
    pub well_1_intensity: f32,
    pub well_2_intensity: f32,
    pub well_3_intensity: f32,
    pub well_4_intensity: f32,
    pub well_5_intensity: f32,
    pub well_6_intensity: f32,
    pub well_7_intensity: f32,
    pub well_8_intensity: f32,
    pub well_9_intensity: f32,
    pub well_10_intensity: f32,
    pub well_11_intensity: f32,
    pub well_12_intensity: f32,
    pub well_13_intensity: f32,
    pub well_14_intensity: f32,
}
