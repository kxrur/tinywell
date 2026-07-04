use diesel::{QueryResult, RunQueryDsl, SqliteConnection};

use crate::database::new_models::NewExperiment;

pub fn store_experiment(
    conn: &mut SqliteConnection,
    new_experiment: NewExperiment,
) -> QueryResult<usize> {
    use crate::database::schema::experiments::dsl::*;

    diesel::insert_into(experiments)
        .values(&new_experiment)
        .execute(conn)
}
