-- Your SQL goes here
CREATE TABLE experiments (
  id INTEGER PRIMARY KEY
);

CREATE TABLE data (
  id INTEGER PRIMARY KEY,
  experiment_id INTEGER NOT NULL,
  well_temperature_c REAL NOT NULL,
  ambient_temperature_c REAL NOT NULL,
  ambient_pressure_pa REAL NOT NULL,
  ambient_humidity_pct REAL NOT NULL,
  FOREIGN KEY (experiment_id) REFERENCES experiments(id)
);

CREATE TABLE well_readings (
  id INTEGER PRIMARY KEY,
  experiment_id INTEGER NOT NULL,
  wavelength_nm REAL NOT NULL,
  well_1_intensity REAL NOT NULL,
  well_2_intensity REAL NOT NULL,
  well_3_intensity REAL NOT NULL,
  well_4_intensity REAL NOT NULL,
  well_5_intensity REAL NOT NULL,
  well_6_intensity REAL NOT NULL,
  well_7_intensity REAL NOT NULL,
  well_8_intensity REAL NOT NULL,
  well_9_intensity REAL NOT NULL,
  well_10_intensity REAL NOT NULL,
  well_11_intensity REAL NOT NULL,
  well_12_intensity REAL NOT NULL,
  well_13_intensity REAL NOT NULL,
  well_14_intensity REAL NOT NULL,
  FOREIGN KEY (experiment_id) REFERENCES experiments(id)
);
