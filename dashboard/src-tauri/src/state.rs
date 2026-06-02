use std::sync::Arc;

use crate::serial::SerialManager;

pub struct AppState {
    pub db_path: String,
    pub serial: Arc<SerialManager>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            db_path: String::new(),
            serial: Arc::new(SerialManager::default()),
        }
    }
}
