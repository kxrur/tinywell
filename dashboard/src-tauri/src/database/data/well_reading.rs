use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl, SqliteConnection};

use crate::database::models::WellReading;
use crate::database::new_models::NewWellReading;

pub fn store_reading(conn: &mut SqliteConnection, reading: NewWellReading) -> QueryResult<usize> {
    use crate::database::schema::well_readings::dsl::*;

    diesel::insert_into(well_readings)
        .values(&reading)
        .execute(conn)
}

pub fn list_readings_for_experiment(
    conn: &mut SqliteConnection,
    target_experiment_id: i32,
) -> QueryResult<Vec<WellReading>> {
    use crate::database::schema::well_readings::dsl::*;

    well_readings
        .filter(experiment_id.eq(target_experiment_id))
        .order(captured_at_ms.asc())
        .load(conn)
}
