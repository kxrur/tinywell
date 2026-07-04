// @generated automatically by Diesel CLI.

diesel::table! {
    data (id) {
        id -> Nullable<Integer>,
        experiment_id -> Integer,
        well_temperature_c -> Float,
        ambient_temperature_c -> Float,
        ambient_pressure_pa -> Float,
        ambient_humidity_pct -> Float,
    }
}

diesel::table! {
    experiments (id) {
        id -> Nullable<Integer>,
        name -> Text,
    }
}

diesel::table! {
    well_readings (id) {
        id -> Nullable<Integer>,
        experiment_id -> Integer,
        wavelength_nm -> Float,
        well_1_intensity -> Float,
        well_2_intensity -> Float,
        well_3_intensity -> Float,
        well_4_intensity -> Float,
        well_5_intensity -> Float,
        well_6_intensity -> Float,
        well_7_intensity -> Float,
        well_8_intensity -> Float,
        well_9_intensity -> Float,
        well_10_intensity -> Float,
        well_11_intensity -> Float,
        well_12_intensity -> Float,
        well_13_intensity -> Float,
        well_14_intensity -> Float,
    }
}

diesel::joinable!(data -> experiments (experiment_id));
diesel::joinable!(well_readings -> experiments (experiment_id));

diesel::allow_tables_to_appear_in_same_query!(data, experiments, well_readings,);
