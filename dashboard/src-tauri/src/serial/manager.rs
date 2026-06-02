use serde::{Deserialize, Serialize};
use specta::Type;
use std::sync::mpsc;
use std::sync::Mutex;
use std::time::Duration;

use super::client::{start_client, ClientHandle, RequestEnvelope};
use super::errors::SerialError;
use super::protocol::{SerialRequest, SerialResponse};
use log::{debug, info, warn};

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub enum ConnectionStatus {
    Disconnected,
    Connecting,
    Connected { port: String },
}

impl Default for ConnectionStatus {
    fn default() -> Self {
        ConnectionStatus::Disconnected
    }
}

#[derive(Default)]
struct ManagerState {
    port_name: Option<String>,
    status: ConnectionStatus,
    client: Option<ClientHandle>,
}

#[derive(Default)]
pub struct SerialManager {
    inner: Mutex<ManagerState>,
}

impl SerialManager {
    pub fn set_port(&self, port_name: String) {
        let mut state = self.inner.lock().expect("serial manager lock");
        info!("Serial port set: {}", port_name);
        state.port_name = Some(port_name);
    }

    pub fn get_port(&self) -> Option<String> {
        let state = self.inner.lock().expect("serial manager lock");
        state.port_name.clone()
    }

    pub fn status(&self) -> ConnectionStatus {
        let state = self.inner.lock().expect("serial manager lock");
        state.status.clone()
    }

    pub fn connect(&self) -> Result<(), SerialError> {
        let port_name = {
            let state = self.inner.lock().expect("serial manager lock");
            state.port_name.clone().ok_or(SerialError::PortNotSet)?
        };

        let mut state = self.inner.lock().expect("serial manager lock");
        if matches!(state.status, ConnectionStatus::Connected { .. }) {
            warn!("Serial already connected");
            return Ok(());
        }

        state.status = ConnectionStatus::Connecting;
        info!("Serial connecting on port {}", port_name);
        let client = start_client(&port_name)?;
        state.status = ConnectionStatus::Connected { port: port_name };
        state.client = Some(client);
        info!("Serial connected");
        Ok(())
    }

    pub fn disconnect(&self) -> Result<(), SerialError> {
        let mut state = self.inner.lock().expect("serial manager lock");
        if let Some(client) = state.client.take() {
            info!("Serial disconnecting");
            client.stop();
        }
        state.status = ConnectionStatus::Disconnected;
        info!("Serial disconnected");
        Ok(())
    }

    pub fn send_request(
        &self,
        request: SerialRequest,
        timeout: Duration,
    ) -> Result<SerialResponse, SerialError> {
        debug!("Serial request queued: {:?}", request);
        let sender = {
            let state = self.inner.lock().expect("serial manager lock");
            let client = state.client.as_ref().ok_or(SerialError::NotConnected)?;
            client.sender()
        };

        let (reply_tx, reply_rx) = mpsc::channel();
        let envelope = RequestEnvelope {
            request,
            timeout,
            reply: reply_tx,
        };

        sender
            .send(envelope)
            .map_err(|_| SerialError::ChannelClosed)?;

        let response = reply_rx.recv().map_err(|_| SerialError::ChannelClosed)?;
        debug!("Serial response received");
        response
    }

    pub fn list_ports(&self) -> Result<Vec<String>, SerialError> {
        let ports =
            serialport::available_ports().map_err(|err| SerialError::IoError(err.to_string()))?;
        info!("Serial ports found: {}", ports.len());
        Ok(ports.into_iter().map(|info| info.port_name).collect())
    }
}
