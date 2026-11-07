#include <Arduino.h>

// Serial-only mroc simulator for exercising the dashboard without the payload.
// The wire protocol is documented in ../serial_protocol_v2.md.

static const uint8_t FRAME_DELIM = 0xB3;
static const uint8_t FRAME_ERROR = 0xE0;
static const uint8_t CMD_NEXT_EXPERIMENT_PHASE = 0x01;
static const uint8_t CMD_RUN_EXPERIMENT_PHASE = 0x02;
static const uint8_t CMD_SET_LED_STATE = 0x03;
static const uint8_t CMD_TEST_SYSTEM_HEALTH = 0x04;
static const uint8_t CMD_SET_STATE_IDLE = 0x05;
static const uint8_t CMD_RESET = 0x55;
static const uint8_t CMD_SYSTEM_STATUS = 0x80;
static const uint8_t CMD_EXPERIMENT_STATUS = 0x81;
static const uint8_t CMD_HARDWARE_HEALTH_STATUS = 0x82;
static const uint8_t CMD_ENVIRONMENT_INFORMATION = 0x83;
static const uint8_t CMD_PHOTOSENSOR_RESULTS = 0x84;
static const uint8_t CMD_TELECOMMAND_ACKNOWLEDGE = 0x85;

static const uint8_t ERR_UNKNOWN_FRAME_ID = 0x01;
static const uint8_t ERR_INVALID_ARGS = 0x03;

static const uint8_t TC_SUCCESS = 0x00;
static const uint8_t TC_NOT_IN_IDLE_MODE = 0x01;
static const uint8_t TC_UNKNOWN_EXPERIMENT_PHASE = 0x03;

static const uint8_t STATE_IDLE = 0;
static const uint8_t STATE_PREPARATION = 1;
static const uint8_t STATE_ACTIVATION = 2;
static const uint8_t STATE_GROWTH = 3;
static const uint8_t STATE_INDUCTION = 4;

static const uint8_t LED_ACTION_OFF = 0;
static const uint8_t LED_ACTION_ON = 1;
static const uint8_t LED_ACTION_TOGGLE = 2;
static const uint8_t LED_ACTION_SET_BRIGHTNESS = 3;

static const uint8_t SERIAL_MAX_PAYLOAD = 3;
static const uint8_t SENSOR_COUNT = 14;
static const uint32_t PREPARATION_DURATION_MS = 20000UL;
static const uint32_t ACTIVATION_DURATION_MS = 120000UL;

static uint32_t bootMillis = 0;
static uint8_t currentState = STATE_IDLE;
static uint8_t lastState = STATE_IDLE;
static uint8_t phaseProgress = 1;
static uint32_t phaseStartedMillis = 0;
static uint32_t phaseStartedAtUptime = 0;
static uint32_t lastPhaseElapsed = 0;
static uint8_t selectedWavelength = 0;
static bool ledEnabled[4] = {false, false, false, false};
static bool ledForcedOff[4] = {false, false, false, false};
static uint8_t ledBrightness[4] = {255, 255, 255, 255};

enum ParserState : uint8_t {
  WAIT_DELIM,
  READ_ID,
  READ_PAYLOAD,
  READ_CHECKSUM
};

static ParserState parserState = WAIT_DELIM;
static uint8_t frameId = 0;
static uint8_t payload[SERIAL_MAX_PAYLOAD] = {0};
static uint8_t payloadLength = 0;
static uint8_t expectedLength = 0;

static uint32_t uptimeSeconds() {
  return (millis() - bootMillis) / 1000UL;
}

static uint8_t checksumFor(uint8_t id, const uint8_t *data, uint8_t length) {
  uint16_t sum = id;
  for (uint8_t i = 0; i < length; ++i) {
    sum += data[i];
  }
  return (uint8_t)(sum & 0xFF);
}

static void writeFrame(uint8_t id, const uint8_t *data, uint8_t length) {
  Serial.write(FRAME_DELIM);
  Serial.write(id);
  if (length > 0) {
    Serial.write(data, length);
  }
  Serial.write(checksumFor(id, data, length));
}

static void sendError(uint8_t code) {
  writeFrame(FRAME_ERROR, &code, 1);
}

static void sendAck(uint8_t commandId, uint8_t result) {
  uint8_t ack[2] = {commandId, result};
  writeFrame(CMD_TELECOMMAND_ACKNOWLEDGE, ack, sizeof(ack));
}

static void writeU16Le(uint8_t *out, uint16_t value) {
  out[0] = (uint8_t)(value & 0xFF);
  out[1] = (uint8_t)((value >> 8) & 0xFF);
}

static void writeU32Le(uint8_t *out, uint32_t value) {
  out[0] = (uint8_t)(value & 0xFF);
  out[1] = (uint8_t)((value >> 8) & 0xFF);
  out[2] = (uint8_t)((value >> 16) & 0xFF);
  out[3] = (uint8_t)((value >> 24) & 0xFF);
}

static uint8_t requestPayloadLength(uint8_t id) {
  switch (id) {
    case 0x00:
    case CMD_NEXT_EXPERIMENT_PHASE:
    case CMD_TEST_SYSTEM_HEALTH:
    case CMD_SET_STATE_IDLE:
    case CMD_RESET:
    case CMD_SYSTEM_STATUS:
    case CMD_EXPERIMENT_STATUS:
    case CMD_HARDWARE_HEALTH_STATUS:
    case CMD_ENVIRONMENT_INFORMATION:
    case CMD_PHOTOSENSOR_RESULTS:
      return 0;
    case CMD_RUN_EXPERIMENT_PHASE:
      return 1;
    case CMD_SET_LED_STATE:
      return 3;
    default:
      return 0xFF;
  }
}

static void stopSimulatedOutputs() {
  for (uint8_t i = 0; i < 4; ++i) {
    ledEnabled[i] = false;
    ledForcedOff[i] = false;
  }
}

static void startPhase(uint8_t phase) {
  currentState = phase;
  lastState = phase;
  phaseProgress = 0;
  phaseStartedMillis = millis();
  phaseStartedAtUptime = uptimeSeconds();
  lastPhaseElapsed = 0;
  if (phase == STATE_ACTIVATION) {
    selectedWavelength = 2;
  } else if (phase == STATE_INDUCTION) {
    selectedWavelength = 0;
  }
}

static void setIdle(bool completed) {
  if (currentState != STATE_IDLE) {
    lastPhaseElapsed = (millis() - phaseStartedMillis) / 1000UL;
    if (completed) {
      phaseProgress = 1;
    }
  }
  currentState = STATE_IDLE;
  stopSimulatedOutputs();
}

static void resetSimulation() {
  bootMillis = millis();
  currentState = STATE_IDLE;
  lastState = STATE_IDLE;
  phaseProgress = 1;
  phaseStartedMillis = bootMillis;
  phaseStartedAtUptime = 0;
  lastPhaseElapsed = 0;
  selectedWavelength = 0;
  stopSimulatedOutputs();
  for (uint8_t i = 0; i < 4; ++i) {
    ledBrightness[i] = 255;
  }
}

static uint32_t activePhaseDurationMs() {
  if (currentState == STATE_PREPARATION) {
    return PREPARATION_DURATION_MS;
  }
  if (currentState == STATE_ACTIVATION) {
    return ACTIVATION_DURATION_MS;
  }
  return 0;
}

static void updateExperiment() {
  const uint32_t duration = activePhaseDurationMs();
  if (duration == 0 || currentState == STATE_IDLE) {
    return;
  }

  if ((uint32_t)(millis() - phaseStartedMillis) >= duration) {
    if (currentState == STATE_ACTIVATION) {
      selectedWavelength = 2;
    }
    setIdle(true);
  }
}

static uint8_t activeWavelength() {
  if (currentState == STATE_GROWTH) {
    const uint32_t capture = (millis() - phaseStartedMillis) / 1000UL;
    return (capture & 1U) ? 3 : 2;
  }
  if (currentState == STATE_INDUCTION) {
    return 0;
  }
  return selectedWavelength;
}

static bool wavelengthLedIsOn(uint8_t wavelength) {
  if (ledForcedOff[wavelength]) {
    return false;
  }
  if (ledEnabled[wavelength]) {
    return true;
  }
  if (currentState == STATE_GROWTH || currentState == STATE_INDUCTION) {
    return wavelength == activeWavelength();
  }
  return false;
}

static void respondSystemStatus() {
  uint8_t data[5] = {0};
  const uint8_t firmwareVersion = 1;
  data[0] = (uint8_t)((firmwareVersion << 3) | (currentState & 0x07));
  writeU32Le(&data[1], uptimeSeconds());
  writeFrame(CMD_SYSTEM_STATUS, data, sizeof(data));
}

static void respondExperimentStatus() {
  uint8_t data[5] = {0};
  const uint32_t phaseTime = currentState == STATE_IDLE
                                 ? lastPhaseElapsed
                                 : phaseStartedAtUptime;
  data[0] = (uint8_t)((lastState << 3) | (phaseProgress & 0x07));
  writeU32Le(&data[1], phaseTime);
  writeFrame(CMD_EXPERIMENT_STATUS, data, sizeof(data));
}

static void respondHardwareHealth() {
  uint8_t data[6] = {0};
  const uint32_t tick = uptimeSeconds();
  const uint16_t bipump = currentState == STATE_PREPARATION
                              ? (uint16_t)(2100 + (tick % 80))
                              : (uint16_t)(24 + (tick % 12));
  const uint16_t heater = 32;
  const uint16_t powerSupply = (uint16_t)(2050 + (tick % 9));
  writeU16Le(&data[0], bipump);
  writeU16Le(&data[2], heater);
  writeU16Le(&data[4], powerSupply);
  writeFrame(CMD_HARDWARE_HEALTH_STATUS, data, sizeof(data));
}

static void respondEnvironment() {
  uint8_t data[14] = {0};
  const uint32_t tick = uptimeSeconds();
  int16_t ambientTemp = (int16_t)(2210 + (tick % 65));
  if (currentState == STATE_ACTIVATION) {
    ambientTemp += 15;
  }
  const uint16_t wellTemp = (uint16_t)(24 + (tick / 12) % 5);
  const uint32_t pressurePa = 101200UL + (tick % 100UL);
  const uint32_t pressureQ24_8 = pressurePa * 256UL;
  const uint32_t humidityQ22_10 = 44000UL + (tick * 13UL % 2200UL);

  writeU16Le(&data[0], wellTemp);
  writeU32Le(&data[2], (uint32_t)(int32_t)ambientTemp);
  writeU32Le(&data[6], pressureQ24_8);
  writeU32Le(&data[10], humidityQ22_10);
  writeFrame(CMD_ENVIRONMENT_INFORMATION, data, sizeof(data));
}

static void respondPhotosensors() {
  uint8_t data[57] = {0};
  const uint8_t wavelength = activeWavelength();
  const uint32_t tick = millis() / 500UL;
  const bool illuminated = wavelengthLedIsOn(wavelength);
  const uint16_t brightnessScale = ledBrightness[wavelength];
  data[0] = wavelength;

  for (uint8_t i = 0; i < SENSOR_COUNT; ++i) {
    uint32_t reading = 2 + ((i * 7 + tick) % 5);
    if (illuminated && brightnessScale > 0) {
      const uint32_t pattern = 700UL + ((i * 193UL + wavelength * 421UL + tick * 11UL) % 2400UL);
      reading = (pattern * brightnessScale) / 255UL;
    }
    writeU32Le(&data[1 + i * 4], reading);
  }

  writeFrame(CMD_PHOTOSENSOR_RESULTS, data, sizeof(data));
}

static void handleLedCommand(const uint8_t *args) {
  const uint8_t wavelength = args[0];
  const uint8_t action = args[1];
  const uint8_t brightness = args[2];

  if (wavelength > 3 || action > LED_ACTION_SET_BRIGHTNESS) {
    sendError(ERR_INVALID_ARGS);
    return;
  }

  selectedWavelength = wavelength;
  switch (action) {
    case LED_ACTION_OFF:
      ledEnabled[wavelength] = false;
      ledForcedOff[wavelength] = true;
      break;
    case LED_ACTION_ON:
      ledEnabled[wavelength] = true;
      ledForcedOff[wavelength] = false;
      break;
    case LED_ACTION_TOGGLE: {
      const bool wasOn = wavelengthLedIsOn(wavelength);
      ledEnabled[wavelength] = !wasOn;
      ledForcedOff[wavelength] = wasOn;
      break;
    }
    case LED_ACTION_SET_BRIGHTNESS:
      ledBrightness[wavelength] = brightness;
      ledEnabled[wavelength] = brightness > 0;
      ledForcedOff[wavelength] = brightness == 0;
      break;
  }

  sendAck(CMD_SET_LED_STATE, TC_SUCCESS);
}

static void handleTelecommand(uint8_t id, const uint8_t *args) {
  switch (id) {
    case CMD_NEXT_EXPERIMENT_PHASE:
      if (currentState != STATE_IDLE) {
        sendAck(id, TC_NOT_IN_IDLE_MODE);
      } else if (lastState >= STATE_INDUCTION) {
        sendAck(id, TC_UNKNOWN_EXPERIMENT_PHASE);
      } else {
        startPhase(lastState + 1);
        sendAck(id, TC_SUCCESS);
      }
      break;
    case CMD_RUN_EXPERIMENT_PHASE:
      if (currentState != STATE_IDLE) {
        sendAck(id, TC_NOT_IN_IDLE_MODE);
      } else if (args[0] < STATE_PREPARATION || args[0] > STATE_INDUCTION) {
        sendAck(id, TC_UNKNOWN_EXPERIMENT_PHASE);
      } else {
        startPhase(args[0]);
        sendAck(id, TC_SUCCESS);
      }
      break;
    case CMD_SET_LED_STATE:
      handleLedCommand(args);
      break;
    case CMD_TEST_SYSTEM_HEALTH:
      sendAck(id, TC_SUCCESS);
      break;
    case CMD_SET_STATE_IDLE:
      setIdle(false);
      sendAck(id, TC_SUCCESS);
      break;
    case CMD_RESET:
      resetSimulation();
      sendAck(id, TC_SUCCESS);
      break;
  }
}

static void handleFrame(uint8_t id, const uint8_t *data, uint8_t length) {
  updateExperiment();
  switch (id) {
    case 0x00:
      writeFrame(0x00, NULL, 0);
      break;
    case CMD_NEXT_EXPERIMENT_PHASE:
    case CMD_RUN_EXPERIMENT_PHASE:
    case CMD_SET_LED_STATE:
    case CMD_TEST_SYSTEM_HEALTH:
    case CMD_SET_STATE_IDLE:
    case CMD_RESET:
      handleTelecommand(id, data);
      break;
    case CMD_SYSTEM_STATUS:
      respondSystemStatus();
      break;
    case CMD_EXPERIMENT_STATUS:
      respondExperimentStatus();
      break;
    case CMD_HARDWARE_HEALTH_STATUS:
      respondHardwareHealth();
      break;
    case CMD_ENVIRONMENT_INFORMATION:
      respondEnvironment();
      break;
    case CMD_PHOTOSENSOR_RESULTS:
      respondPhotosensors();
      break;
    default:
      (void)length;
      sendError(ERR_UNKNOWN_FRAME_ID);
      break;
  }
}

static void pollSerial() {
  while (Serial.available() > 0) {
    const uint8_t incoming = (uint8_t)Serial.read();
    switch (parserState) {
      case WAIT_DELIM:
        if (incoming == FRAME_DELIM) {
          parserState = READ_ID;
        }
        break;
      case READ_ID:
        frameId = incoming;
        expectedLength = requestPayloadLength(frameId);
        payloadLength = 0;
        if (expectedLength == 0xFF) {
          sendError(ERR_UNKNOWN_FRAME_ID);
          parserState = WAIT_DELIM;
        } else if (expectedLength == 0) {
          parserState = READ_CHECKSUM;
        } else {
          parserState = READ_PAYLOAD;
        }
        break;
      case READ_PAYLOAD:
        if (payloadLength < SERIAL_MAX_PAYLOAD) {
          payload[payloadLength++] = incoming;
        }
        if (payloadLength >= expectedLength) {
          parserState = READ_CHECKSUM;
        }
        break;
      case READ_CHECKSUM:
        if (incoming == checksumFor(frameId, payload, expectedLength)) {
          handleFrame(frameId, payload, expectedLength);
        }
        parserState = WAIT_DELIM;
        break;
    }
  }
}

void setup() {
  Serial.begin(115200);
  bootMillis = millis();
  phaseStartedMillis = bootMillis;
}

void loop() {
  updateExperiment();
  pollSerial();
}
