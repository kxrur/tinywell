use std::io::{Read, Write};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::{Duration, Instant};

use serialport::SerialPort;

use super::errors::SerialError;
use super::protocol::{
    build_frame, request_frame, response_from_frame, Parser, SerialRequest, SerialResponse,
    FRAME_ENVIRONMENT_INFO, FRAME_ERROR, FRAME_PHOTOSENSOR_RESULTS, FRAME_PING,
    FRAME_SYSTEM_STATUS, FRAME_TELECOMMAND_ACK,
};
use log::{debug, error, info};

pub struct RequestEnvelope {
    pub request: SerialRequest,
    pub timeout: Duration,
    pub reply: Sender<Result<SerialResponse, SerialError>>,
}

pub struct ClientHandle {
    sender: Sender<RequestEnvelope>,
    stop: Sender<()>,
    join_handle: Option<thread::JoinHandle<()>>,
    port_name: String,
}

impl ClientHandle {
    pub fn sender(&self) -> Sender<RequestEnvelope> {
        self.sender.clone()
    }

    pub fn port_name(&self) -> &str {
        &self.port_name
    }

    pub fn stop(mut self) {
        let _ = self.stop.send(());
        if let Some(handle) = self.join_handle.take() {
            let _ = handle.join();
        }
    }
}

pub fn start_client(port_name: &str) -> Result<ClientHandle, SerialError> {
    let (request_tx, request_rx) = mpsc::channel();
    let (stop_tx, stop_rx) = mpsc::channel();
    let port_name_string = port_name.to_string();

    let join_handle = thread::spawn({
        let port_name = port_name.to_string();
        move || {
            info!("Serial client loop starting on {}", port_name);
            if let Err(err) = client_loop(&port_name, request_rx, stop_rx) {
                error!("Serial client stopped: {}", err);
            }
            info!("Serial client loop stopped");
        }
    });

    Ok(ClientHandle {
        sender: request_tx,
        stop: stop_tx,
        join_handle: Some(join_handle),
        port_name: port_name_string,
    })
}

fn client_loop(
    port_name: &str,
    request_rx: Receiver<RequestEnvelope>,
    stop_rx: Receiver<()>,
) -> Result<(), SerialError> {
    info!("Opening serial port {}", port_name);
    let mut port = serialport::new(port_name, 115_200)
        .timeout(Duration::from_millis(50))
        .open()
        .map_err(|err| SerialError::OpenFailed(err.to_string()))?;

    info!("Serial port opened {}", port_name);

    let mut parser = Parser::default();

    loop {
        if stop_rx.try_recv().is_ok() {
            break;
        }

        match request_rx.recv_timeout(Duration::from_millis(50)) {
            Ok(envelope) => {
                debug!("Serial handling request");
                let result = handle_request(&mut *port, &mut parser, &envelope);
                let _ = envelope.reply.send(result);
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {
                continue;
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => break,
        }
    }

    Ok(())
}

fn handle_request(
    port: &mut dyn SerialPort,
    parser: &mut Parser,
    envelope: &RequestEnvelope,
) -> Result<SerialResponse, SerialError> {
    let (frame_id, payload) = request_frame(&envelope.request);
    let frame_bytes = build_frame(frame_id, &payload);

    port.write_all(&frame_bytes)
        .map_err(|err| SerialError::IoError(err.to_string()))?;
    port.flush()
        .map_err(|err| SerialError::IoError(err.to_string()))?;

    let expected_response = expected_response_id(&envelope.request);
    let start = Instant::now();
    let mut buf = [0u8; 128];

    while start.elapsed() < envelope.timeout {
        match port.read(&mut buf) {
            Ok(count) if count > 0 => {
                let frames = parser.push_bytes(&buf[..count]);
                for frame in frames {
                    if frame.frame_id == FRAME_ERROR {
                        let code = frame.payload.get(0).copied().unwrap_or(0);
                        debug!("Serial device error 0x{:02X}", code);
                        return Err(SerialError::DeviceError(code));
                    }
                    if frame.frame_id == expected_response {
                        let response = response_from_frame(frame);
                        debug!("Serial response matched");
                        return match response {
                            SerialResponse::Unknown { .. } => Err(SerialError::Protocol(
                                "Unexpected response payload".to_string(),
                            )),
                            SerialResponse::Error { code } => Err(SerialError::DeviceError(code)),
                            _ => Ok(response),
                        };
                    }
                }
            }
            Ok(_) => continue,
            Err(err) if err.kind() == std::io::ErrorKind::TimedOut => continue,
            Err(err) => return Err(SerialError::IoError(err.to_string())),
        }
    }

    Err(SerialError::Timeout)
}

fn expected_response_id(request: &SerialRequest) -> u8 {
    match request {
        SerialRequest::Ping => FRAME_PING,
        SerialRequest::SystemStatus => FRAME_SYSTEM_STATUS,
        SerialRequest::EnvironmentInfo => FRAME_ENVIRONMENT_INFO,
        SerialRequest::PhotosensorResults => FRAME_PHOTOSENSOR_RESULTS,
        SerialRequest::SetLedState { .. } => FRAME_TELECOMMAND_ACK,
    }
}
