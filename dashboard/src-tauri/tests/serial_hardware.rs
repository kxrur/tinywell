use std::env;
use std::sync::Once;
use std::time::{Duration, Instant};

use tinywell_lib::serial::{LedAction, SerialManager, SerialRequest, SerialResponse};

const REQUEST_TIMEOUT: Duration = Duration::from_secs(2);
const DEVICE_READY_TIMEOUT: Duration = Duration::from_secs(10);
const RETRY_DELAY: Duration = Duration::from_millis(250);

fn init_logging() {
    static LOGGER: Once = Once::new();
    LOGGER.call_once(|| {
        let _ = env_logger::builder().is_test(true).try_init();
    });
}

struct SerialSession {
    manager: SerialManager,
}

impl SerialSession {
    fn connect() -> Self {
        init_logging();
        let port = env::var("TINYWELL_SERIAL_PORT")
            .expect("Set TINYWELL_SERIAL_PORT to the connected device port");
        let manager = SerialManager::default();
        manager.set_port(port);
        manager.connect().expect("serial connection should start");
        let session = Self { manager };
        session.wait_until_ready();
        session
    }

    fn wait_until_ready(&self) {
        let deadline = Instant::now() + DEVICE_READY_TIMEOUT;
        let mut last_error = None;

        while Instant::now() < deadline {
            match self
                .manager
                .send_request(SerialRequest::Ping, REQUEST_TIMEOUT)
            {
                Ok(SerialResponse::Ping) => return,
                Ok(response) => {
                    last_error = Some(format!("unexpected Ping response: {response:?}"));
                }
                Err(error) => last_error = Some(error.to_string()),
            }
            std::thread::sleep(RETRY_DELAY);
        }

        panic!(
            "device did not become ready within {} seconds; last Ping error: {}",
            DEVICE_READY_TIMEOUT.as_secs(),
            last_error.unwrap_or_else(|| "no response".to_string())
        );
    }

    fn request(&self, request: SerialRequest) -> SerialResponse {
        self.manager
            .send_request(request, REQUEST_TIMEOUT)
            .unwrap_or_else(|error| panic!("device should return the documented response: {error}"))
    }
}

impl Drop for SerialSession {
    fn drop(&mut self) {
        let _ = self.manager.disconnect();
    }
}

#[test]
#[ignore]
fn ping_route_responds() {
    let device = SerialSession::connect();
    assert!(matches!(
        device.request(SerialRequest::Ping),
        SerialResponse::Ping
    ));
}

#[test]
#[ignore]
fn system_status_route_responds() {
    let device = SerialSession::connect();
    assert!(matches!(
        device.request(SerialRequest::SystemStatus),
        SerialResponse::SystemStatus { .. }
    ));
}

#[test]
#[ignore]
fn experiment_status_route_responds() {
    let device = SerialSession::connect();
    assert!(matches!(
        device.request(SerialRequest::ExperimentStatus),
        SerialResponse::ExperimentStatus { .. }
    ));
}

#[test]
#[ignore]
fn hardware_health_route_responds() {
    let device = SerialSession::connect();
    assert!(matches!(
        device.request(SerialRequest::HardwareHealth),
        SerialResponse::HardwareHealth { .. }
    ));
}

#[test]
#[ignore]
fn environment_info_route_responds() {
    let device = SerialSession::connect();
    assert!(matches!(
        device.request(SerialRequest::EnvironmentInfo),
        SerialResponse::EnvironmentInfo { .. }
    ));
}

#[test]
#[ignore]
fn photosensor_results_route_returns_all_wells() {
    let device = SerialSession::connect();
    match device.request(SerialRequest::PhotosensorResults) {
        SerialResponse::PhotosensorResults { wavelength, values } => {
            assert!(
                wavelength <= 3,
                "unknown wavelength enum value: {wavelength}"
            );
            assert_eq!(values.len(), 14, "expected a reading for every well");
        }
        response => panic!("unexpected photosensor response: {response:?}"),
    }
}

#[test]
#[ignore]
fn set_led_state_route_acknowledges() {
    let device = SerialSession::connect();
    match device.request(SerialRequest::SetLedState {
        wavelength: 0,
        action: LedAction::SetBrightness,
        brightness: 0,
    }) {
        SerialResponse::TelecommandAck { tc_id, tc_result } => {
            assert_eq!(tc_id, 0x03, "acknowledgement should identify Set LED State");
            assert!(tc_result <= 0x07, "unknown telecommand result: {tc_result}");
        }
        response => panic!("unexpected Set LED State response: {response:?}"),
    }
}
