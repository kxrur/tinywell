use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl, SqliteConnection};

use crate::database::models::Data;
use crate::database::new_models::NewData;

pub fn store_data(conn: &mut SqliteConnection, new_data: NewData) -> QueryResult<usize> {
    use crate::database::schema::data::dsl::*;

    diesel::insert_into(data).values(&new_data).execute(conn)
}

pub fn list_data_for_experiment(
    conn: &mut SqliteConnection,
    target_experiment_id: i32,
) -> QueryResult<Vec<Data>> {
    use crate::database::schema::data::dsl::*;

    data.filter(experiment_id.eq(target_experiment_id))
        .order(captured_at_ms.asc())
        .load(conn)
}

pub fn list_recent_data_for_experiment(
    conn: &mut SqliteConnection,
    target_experiment_id: i32,
    max_rows: i64,
) -> QueryResult<Vec<Data>> {
    use crate::database::schema::data::dsl::*;

    let mut rows: Vec<Data> = data
        .filter(experiment_id.eq(target_experiment_id))
        .order(captured_at_ms.desc())
        .limit(max_rows)
        .load(conn)?;
    rows.reverse();
    Ok(rows)
}
