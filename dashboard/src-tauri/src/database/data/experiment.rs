use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl, SqliteConnection};

use crate::database::models::Experiment;
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

pub fn list_experiments(conn: &mut SqliteConnection) -> QueryResult<Vec<Experiment>> {
    use crate::database::schema::experiments::dsl::*;

    experiments.order(id.desc()).load(conn)
}

pub fn experiment_exists(conn: &mut SqliteConnection, target_id: i32) -> QueryResult<bool> {
    use crate::database::schema::experiments::dsl::*;

    diesel::select(diesel::dsl::exists(experiments.filter(id.eq(target_id)))).get_result(conn)
}

pub fn experiment_name_exists(conn: &mut SqliteConnection, target_name: &str) -> QueryResult<bool> {
    use crate::database::schema::experiments::dsl::*;

    diesel::select(diesel::dsl::exists(
        experiments.filter(name.eq(target_name)),
    ))
    .get_result(conn)
}
