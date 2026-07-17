use std::sync::Arc;

use diesel::SqliteConnection;

use crate::serial::SerialManager;

#[derive(Default)]
pub struct AppState {
    pub db_path: String,
    pub db_connection: Option<SqliteConnection>,
    pub active_experiment_id: Option<i32>,
    pub serial: Arc<SerialManager>,
}

impl AppState {
    pub fn db_connection_mut(&mut self) -> Result<&mut SqliteConnection, String> {
        self.db_connection
            .as_mut()
            .ok_or_else(|| "Database is not initialized".to_string())
    }
}
