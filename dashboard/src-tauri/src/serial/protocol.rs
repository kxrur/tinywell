use serde::{Deserialize, Serialize};
use specta::Type;

pub const FRAME_DELIM: u8 = 0xB3;
pub const FRAME_ERROR: u8 = 0xE0;

pub const FRAME_PING: u8 = 0x00;
pub const FRAME_SYSTEM_STATUS: u8 = 0x80;
pub const FRAME_ENVIRONMENT_INFO: u8 = 0x83;
pub const FRAME_PHOTOSENSOR_RESULTS: u8 = 0x84;
pub const FRAME_SET_LED_STATE: u8 = 0x03;
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
    SystemStatus,
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
    let mut out = Vec::with_capacity(3 + payload.len());
    out.push(FRAME_DELIM);
    out.push(frame_id);
    out.extend_from_slice(payload);
    out.push(checksum(frame_id, payload));
    out
}

pub fn expected_payload_len(frame_id: u8) -> Option<usize> {
    match frame_id {
        FRAME_PING => Some(0),
        FRAME_SYSTEM_STATUS => Some(5),
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
        SerialRequest::SystemStatus => (FRAME_SYSTEM_STATUS, Vec::new()),
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
