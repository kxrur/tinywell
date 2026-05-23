use crate::serial::serial::Serial;


#[derive(Default)]
pub struct AppState {
    pub db_path: String,
    pub serial: Serial
}