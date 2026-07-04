use diesel::{QueryResult, RunQueryDsl, SqliteConnection};

use crate::database::new_models::NewWellReading;

pub fn store_reading(conn: &mut SqliteConnection, reading: NewWellReading) -> QueryResult<usize> {
    use crate::database::schema::well_readings::dsl::*;

    diesel::insert_into(well_readings)
        .values(&reading)
        .execute(conn)
}
