use diesel::{QueryResult, RunQueryDsl, SqliteConnection};

use crate::database::new_models::NewData;

pub fn store_data(conn: &mut SqliteConnection, new_data: NewData) -> QueryResult<usize> {
    use crate::database::schema::data::dsl::*;

    diesel::insert_into(data).values(&new_data).execute(conn)
}
