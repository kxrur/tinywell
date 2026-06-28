use std::sync::Arc;

use diesel::SqliteConnection;

use crate::serial::SerialManager;

#[derive(Default)]
pub struct AppState {
    pub db_path: String,
    pub db_connection: Option<SqliteConnection>,
    pub serial: Arc<SerialManager>,
}
