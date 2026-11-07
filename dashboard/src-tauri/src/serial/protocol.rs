use serde::{Deserialize, Serialize};
use specta::Type;

pub const FRAME_DELIM: u8 = 0xB3;
pub const FRAME_ERROR: u8 = 0xE0;
pub const FRAME_TERMINATOR: u8 = b'\n';

pub const FRAME_PING: u8 = 0x00;
pub const FRAME_SYSTEM_STATUS: u8 = 0x80;
pub const FRAME_EXPERIMENT_STATUS: u8 = 0x81;
pub const FRAME_HARDWARE_HEALTH: u8 = 0x82;
pub const FRAME_ENVIRONMENT_INFO: u8 = 0x83;
pub const FRAME_PHOTOSENSOR_RESULTS: u8 = 0x84;
pub const FRAME_NEXT_EXPERIMENT_PHASE: u8 = 0x01;
pub const FRAME_RUN_EXPERIMENT_PHASE: u8 = 0x02;
pub const FRAME_SET_LED_STATE: u8 = 0x03;
pub const FRAME_TEST_SYSTEM_HEALTH: u8 = 0x04;
pub const FRAME_SET_STATE_IDLE: u8 = 0x05;
pub const FRAME_RESET: u8 = 0x55;
pub const FRAME_TELECOMMAND_ACK: u8 = 0x85;

#[derive(Debug, Clone)]
pub struct Frame {
    pub frame_id: u8,
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub enum LedAction {
    Off,
    On,
    Toggle,
    SetBrightness,
}

impl LedAction {
    pub fn as_u8(&self) -> u8 {
        match self {
            LedAction::Off => 0,
            LedAction::On => 1,
            LedAction::Toggle => 2,
            LedAction::SetBrightness => 3,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub enum SerialRequest {
    Ping,
    NextExperimentPhase,
    RunExperimentPhase {
        phase: u8,
    },
    TestSystemHealth,
    SetStateIdle,
    Reset,
    SystemStatus,
    ExperimentStatus,
    HardwareHealth,
    EnvironmentInfo,
    PhotosensorResults,
    SetLedState {
        wavelength: u8,
        action: LedAction,
        brightness: u8,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub enum SerialResponse {
    Ping,
    SystemStatus {
        fw: u8,
        state: u8,
        uptime: u32,
    },
    ExperimentStatus {
        last_state: u8,
        progress: u8,
        elapsed_or_started_at_uptime: u32,
    },
    HardwareHealth {
        bipump_current: u16,
        heater_current: u16,
        power_supply_sense: u16,
    },
    EnvironmentInfo {
        well_temp: u16,
        ambient_temp: i32,
        ambient_pressure: u32,
        ambient_humidity: u32,
    },
    PhotosensorResults {
        wavelength: u8,
        values: Vec<u32>,
    },
    TelecommandAck {
        tc_id: u8,
        tc_result: u8,
    },
    Error {
        code: u8,
    },
    Unknown {
        frame_id: u8,
        payload: Vec<u8>,
    },
}

pub fn checksum(frame_id: u8, payload: &[u8]) -> u8 {
    let mut sum: u16 = frame_id as u16;
    for byte in payload {
        sum = sum.wrapping_add(*byte as u16);
    }
    (sum & 0xFF) as u8
}

pub fn build_frame(frame_id: u8, payload: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(4 + payload.len());
    out.push(FRAME_DELIM);
    out.push(frame_id);
    out.extend_from_slice(payload);
    out.push(checksum(frame_id, payload));
    out.push(FRAME_TERMINATOR);
    out
}

pub fn expected_payload_len(frame_id: u8) -> Option<usize> {
    match frame_id {
        FRAME_PING => Some(0),
        FRAME_SYSTEM_STATUS => Some(5),
        FRAME_EXPERIMENT_STATUS => Some(5),
        FRAME_HARDWARE_HEALTH => Some(6),
        FRAME_ENVIRONMENT_INFO => Some(14),
        FRAME_PHOTOSENSOR_RESULTS => Some(57),
        FRAME_TELECOMMAND_ACK => Some(2),
        FRAME_ERROR => Some(1),
        _ => None,
    }
}

pub fn request_frame(request: &SerialRequest) -> (u8, Vec<u8>) {
    match request {
        SerialRequest::Ping => (FRAME_PING, Vec::new()),
        SerialRequest::NextExperimentPhase => (FRAME_NEXT_EXPERIMENT_PHASE, Vec::new()),
        SerialRequest::RunExperimentPhase { phase } => (FRAME_RUN_EXPERIMENT_PHASE, vec![*phase]),
        SerialRequest::TestSystemHealth => (FRAME_TEST_SYSTEM_HEALTH, Vec::new()),
        SerialRequest::SetStateIdle => (FRAME_SET_STATE_IDLE, Vec::new()),
        SerialRequest::Reset => (FRAME_RESET, Vec::new()),
        SerialRequest::SystemStatus => (FRAME_SYSTEM_STATUS, Vec::new()),
        SerialRequest::ExperimentStatus => (FRAME_EXPERIMENT_STATUS, Vec::new()),
        SerialRequest::HardwareHealth => (FRAME_HARDWARE_HEALTH, Vec::new()),
        SerialRequest::EnvironmentInfo => (FRAME_ENVIRONMENT_INFO, Vec::new()),
        SerialRequest::PhotosensorResults => (FRAME_PHOTOSENSOR_RESULTS, Vec::new()),
        SerialRequest::SetLedState {
            wavelength,
            action,
            brightness,
        } => (
            FRAME_SET_LED_STATE,
            vec![*wavelength, action.as_u8(), *brightness],
        ),
    }
}

pub fn response_from_frame(frame: Frame) -> SerialResponse {
    match frame.frame_id {
        FRAME_PING => SerialResponse::Ping,
        FRAME_SYSTEM_STATUS => {
            if frame.payload.len() == 5 {
                let fw_and_state = frame.payload[0];
                let fw = (fw_and_state >> 3) & 0x1F;
                let state = fw_and_state & 0x07;
                let uptime = read_u32_le(&frame.payload[1..5]);
                SerialResponse::SystemStatus { fw, state, uptime }
            } else {
                SerialResponse::Unknown {
                    frame_id: frame.frame_id,
                    payload: frame.payload,
                }
            }
        }
        FRAME_EXPERIMENT_STATUS => {
            if frame.payload.len() == 5 {
                let last_state_and_progress = frame.payload[0];
                SerialResponse::ExperimentStatus {
                    last_state: (last_state_and_progress >> 3) & 0x1F,
                    progress: last_state_and_progress & 0x07,
                    elapsed_or_started_at_uptime: read_u32_le(&frame.payload[1..5]),
                }
            } else {
                SerialResponse::Unknown {
                    frame_id: frame.frame_id,
                    payload: frame.payload,
                }
            }
        }
        FRAME_HARDWARE_HEALTH => {
            if frame.payload.len() == 6 {
                SerialResponse::HardwareHealth {
                    bipump_current: read_u16_le(&frame.payload[0..2]),
                    heater_current: read_u16_le(&frame.payload[2..4]),
                    power_supply_sense: read_u16_le(&frame.payload[4..6]),
                }
            } else {
                SerialResponse::Unknown {
                    frame_id: frame.frame_id,
                    payload: frame.payload,
                }
            }
        }
        FRAME_ENVIRONMENT_INFO => {
            if frame.payload.len() == 14 {
                let well_temp = read_u16_le(&frame.payload[0..2]);
                let ambient_temp = read_i32_le(&frame.payload[2..6]);
                let ambient_pressure = read_u32_le(&frame.payload[6..10]);
                let ambient_humidity = read_u32_le(&frame.payload[10..14]);
                SerialResponse::EnvironmentInfo {
                    well_temp,
                    ambient_temp,
                    ambient_pressure,
                    ambient_humidity,
                }
            } else {
                SerialResponse::Unknown {
                    frame_id: frame.frame_id,
                    payload: frame.payload,
                }
            }
        }
        FRAME_PHOTOSENSOR_RESULTS => {
            if frame.payload.len() == 57 {
                let wavelength = frame.payload[0];
                let mut values = Vec::with_capacity(14);
                for idx in 0..14 {
                    let start = 1 + (idx * 4);
                    let end = start + 4;
                    values.push(read_u32_le(&frame.payload[start..end]));
                }
                SerialResponse::PhotosensorResults { wavelength, values }
            } else {
                SerialResponse::Unknown {
                    frame_id: frame.frame_id,
                    payload: frame.payload,
                }
            }
        }
        FRAME_TELECOMMAND_ACK => {
            if frame.payload.len() == 2 {
                SerialResponse::TelecommandAck {
                    tc_id: frame.payload[0],
                    tc_result: frame.payload[1],
                }
            } else {
                SerialResponse::Unknown {
                    frame_id: frame.frame_id,
                    payload: frame.payload,
                }
            }
        }
        FRAME_ERROR => SerialResponse::Error {
            code: frame.payload.get(0).copied().unwrap_or(0),
        },
        _ => SerialResponse::Unknown {
            frame_id: frame.frame_id,
            payload: frame.payload,
        },
    }
}

fn read_u16_le(bytes: &[u8]) -> u16 {
    (bytes[0] as u16) | ((bytes[1] as u16) << 8)
}

fn read_u32_le(bytes: &[u8]) -> u32 {
    (bytes[0] as u32)
        | ((bytes[1] as u32) << 8)
        | ((bytes[2] as u32) << 16)
        | ((bytes[3] as u32) << 24)
}

fn read_i32_le(bytes: &[u8]) -> i32 {
    read_u32_le(bytes) as i32
}

#[derive(Debug, Default)]
pub struct Parser {
    state: ParserState,
}

#[derive(Debug)]
enum ParserState {
    WaitDelim,
    ReadId,
    ReadPayload {
        frame_id: u8,
        expected_len: usize,
        payload: Vec<u8>,
    },
    ReadChecksum {
        frame_id: u8,
        payload: Vec<u8>,
    },
}

impl Default for ParserState {
    fn default() -> Self {
        ParserState::WaitDelim
    }
}

impl Parser {
    pub fn push_bytes(&mut self, bytes: &[u8]) -> Vec<Frame> {
        let mut frames = Vec::new();

        for byte in bytes {
            match &mut self.state {
                ParserState::WaitDelim => {
                    if *byte == FRAME_DELIM {
                        self.state = ParserState::ReadId;
                    }
                }
                ParserState::ReadId => {
                    let frame_id = *byte;
                    if let Some(expected_len) = expected_payload_len(frame_id) {
                        if expected_len == 0 {
                            self.state = ParserState::ReadChecksum {
                                frame_id,
                                payload: Vec::new(),
                            };
                        } else {
                            self.state = ParserState::ReadPayload {
                                frame_id,
                                expected_len,
                                payload: Vec::with_capacity(expected_len),
                            };
                        }
                    } else {
                        self.state = ParserState::WaitDelim;
                    }
                }
                ParserState::ReadPayload {
                    frame_id,
                    expected_len,
                    payload,
                } => {
                    payload.push(*byte);
                    if payload.len() >= *expected_len {
                        let frame_id = *frame_id;
                        let payload = std::mem::take(payload);
                        self.state = ParserState::ReadChecksum { frame_id, payload };
                    }
                }
                ParserState::ReadChecksum { frame_id, payload } => {
                    let expected = checksum(*frame_id, payload);
                    if *byte == expected {
                        frames.push(Frame {
                            frame_id: *frame_id,
                            payload: payload.clone(),
                        });
                    }
                    self.state = ParserState::WaitDelim;
                }
            }
        }

        frames
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encodes_all_telecommands_with_spec_ids_and_payloads() {
        let cases = [
            (SerialRequest::NextExperimentPhase, 0x01, vec![]),
            (
                SerialRequest::RunExperimentPhase { phase: 2 },
                0x02,
                vec![2],
            ),
            (
                SerialRequest::SetLedState {
                    wavelength: 0,
                    action: LedAction::SetBrightness,
                    brightness: 128,
                },
                0x03,
                vec![0, 3, 128],
            ),
            (SerialRequest::TestSystemHealth, 0x04, vec![]),
            (SerialRequest::SetStateIdle, 0x05, vec![]),
            (SerialRequest::Reset, 0x55, vec![]),
        ];

        for (request, expected_id, expected_payload) in cases {
            assert_eq!(request_frame(&request), (expected_id, expected_payload));
        }
    }

    #[test]
    fn decodes_hardware_health_with_semantic_field_names() {
        let response = response_from_frame(Frame {
            frame_id: FRAME_HARDWARE_HEALTH,
            payload: vec![0x2C, 0x01, 0x90, 0x01, 0xF4, 0x01],
        });

        assert!(matches!(
            response,
            SerialResponse::HardwareHealth {
                bipump_current: 300,
                heater_current: 400,
                power_supply_sense: 500
            }
        ));
    }

    #[test]
    fn checksum_and_frame_examples_match_protocol_v2() {
        assert_eq!(build_frame(0x02, &[2]), vec![0xB3, 0x02, 0x02, 0x04, b'\n']);
        assert_eq!(
            build_frame(0x85, &[0x02, 0x03]),
            vec![0xB3, 0x85, 0x02, 0x03, 0x8A, b'\n']
        );
    }
}
