// Pico serial protocol mock (no MSP430 required)
// Implements the framed binary protocol used by pico_msp430_i2c.ino

#include <Arduino.h>

// Frame IDs
#define CMD_SET_LED_STATE           0x03
#define CMD_SYSTEM_STATUS           0x80
#define CMD_EXPERIMENT_STATUS       0x81
#define CMD_HARDWARE_HEALTH_STATUS  0x82
#define CMD_ENVIRONMENT_INFORMATION 0x83
#define CMD_PHOTOSENSOR_RESULTS     0x84
#define CMD_TELECOMMAND_ACKNOWLEDGE 0x85

// Serial framing
#define FRAME_DELIM                 0xB3
#define FRAME_ERROR                 0xE0

// Error codes
#define ERR_UNKNOWN_FRAME_ID        0x01
#define ERR_INVALID_LENGTH          0x02
#define ERR_INVALID_ARGS            0x03

// LED telecommand arguments
#define LED_ACTION_OFF              0
#define LED_ACTION_ON               1
#define LED_ACTION_TOGGLE           2
#define LED_ACTION_SET_BRIGHTNESS   3

static const size_t SERIAL_MAX_PAYLOAD = 64;

static uint32_t fakeUptimeStart = 0;
static uint8_t currentWavelength = 0;
static uint8_t ledEnabled[4] = {1, 1, 1, 1};
static uint8_t ledBrightness[4] = {255, 255, 255, 255};

static uint8_t nextEnabledWavelength(uint8_t start) {
  for (uint8_t offset = 0; offset < 4; offset++) {
    uint8_t candidate = (uint8_t)((start + offset) % 4);
    if (ledEnabled[candidate]) {
      return candidate;
    }
  }
  return start;
}

static uint32_t wavelengthSensorPattern(uint8_t wavelength, uint8_t sensorIndex, uint32_t tick) {
  switch (wavelength) {
    case 0:
      return 900 + (((sensorIndex * 220) + tick) % 2200);
    case 1:
      return 900 + ((((13 - sensorIndex) * 220) + tick) % 2200);
    case 2:
      return 1000 + ((((sensorIndex % 2) == 0 ? 1800 : 500) + tick) % 2000);
    case 3:
      return 950 + (((((sensorIndex + 3) % 4) * 520) + tick) % 2100);
    default:
      return 1000 + tick % 2000;
  }
}

static uint8_t checksumFor(uint8_t frameId, const uint8_t *payload, size_t length) {
  uint16_t sum = frameId;
  for (size_t i = 0; i < length; i++) {
    sum += payload[i];
  }
  return (uint8_t)(sum & 0xFF);
}

static void writeFrame(uint8_t frameId, const uint8_t *payload, size_t length) {
  uint8_t checksum = checksumFor(frameId, payload, length);
  Serial.write(FRAME_DELIM);
  Serial.write(frameId);
  if (length > 0) {
    Serial.write(payload, length);
  }
  Serial.write(checksum);
}

static void sendError(uint8_t errorCode) {
  writeFrame(FRAME_ERROR, &errorCode, 1);
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

static uint8_t expectedPayloadLength(uint8_t frameId) {
  switch (frameId) {
    case 0x00:
    case CMD_SYSTEM_STATUS:
    case CMD_EXPERIMENT_STATUS:
    case CMD_HARDWARE_HEALTH_STATUS:
    case CMD_ENVIRONMENT_INFORMATION:
    case CMD_PHOTOSENSOR_RESULTS:
      return 0;
    case CMD_SET_LED_STATE:
      return 3;
    default:
      return 0xFF;
  }
}

static void respondPing() {
  writeFrame(0x00, nullptr, 0);
}

static void respondSys() {
  uint8_t payload[5] = {0};
  uint8_t fw = 3;
  uint8_t state = 1;
  uint32_t uptime = (millis() - fakeUptimeStart) / 1000;
  payload[0] = (uint8_t)((fw << 3) | (state & 0x07));
  writeU32Le(&payload[1], uptime);
  writeFrame(CMD_SYSTEM_STATUS, payload, sizeof(payload));
}

static void respondExperimentStatus() {
  uint8_t payload[5] = {0};
  uint8_t lastState = 2;
  uint8_t progress = 1;
  uint32_t value = 42;
  payload[0] = (uint8_t)((lastState << 3) | (progress & 0x07));
  writeU32Le(&payload[1], value);
  writeFrame(CMD_EXPERIMENT_STATUS, payload, sizeof(payload));
}

static void respondHardwareHealth() {
  uint8_t payload[6] = {0};
  writeU16Le(&payload[0], 300);
  writeU16Le(&payload[2], 400);
  writeU16Le(&payload[4], 500);
  writeFrame(CMD_HARDWARE_HEALTH_STATUS, payload, sizeof(payload));
}

static void respondEnv() {
  uint8_t payload[14] = {0};
  uint16_t wellTemp = 250;
  int32_t ambientTemp = 23000;
  uint32_t pressure = 101325;
  uint32_t humidity = 45000;
  writeU16Le(&payload[0], wellTemp);
  writeU32Le(&payload[2], (uint32_t)ambientTemp);
  writeU32Le(&payload[6], pressure);
  writeU32Le(&payload[10], humidity);
  writeFrame(CMD_ENVIRONMENT_INFORMATION, payload, sizeof(payload));
}

static void respondPhoto() {
  currentWavelength = nextEnabledWavelength((uint8_t)(currentWavelength + 1));

  uint8_t payload[57] = {0};
  payload[0] = currentWavelength;

  uint32_t t = millis();
  uint8_t brightness = ledBrightness[currentWavelength];
  uint32_t brightnessScale = (uint32_t)brightness + 1;
  uint32_t enabledScale = ledEnabled[currentWavelength] ? 1 : 0;
  uint32_t tick = t / 50;

  for (int i = 0; i < 14; i++) {
    uint32_t base = wavelengthSensorPattern(currentWavelength, (uint8_t)i, tick);
    uint32_t value = (base * brightnessScale * enabledScale) / 256;

    writeU32Le(&payload[1 + (i * 4)], value);
  }

  writeFrame(CMD_PHOTOSENSOR_RESULTS, payload, sizeof(payload));
}

static void respondLed(const uint8_t *payload, size_t length) {
  if (length != 3) {
    sendError(ERR_INVALID_LENGTH);
    return;
  }
  uint8_t wavelength = payload[0];
  uint8_t action = payload[1];
  uint8_t brightness = payload[2];
  if (wavelength > 3 || action > LED_ACTION_SET_BRIGHTNESS) {
    sendError(ERR_INVALID_ARGS);
    return;
  }

  currentWavelength = wavelength;
  switch (action) {
    case LED_ACTION_OFF:
      ledEnabled[wavelength] = 0;
      break;
    case LED_ACTION_ON:
      ledEnabled[wavelength] = 1;
      break;
    case LED_ACTION_TOGGLE:
      ledEnabled[wavelength] = ledEnabled[wavelength] ? 0 : 1;
      break;
    case LED_ACTION_SET_BRIGHTNESS:
      ledBrightness[wavelength] = brightness;
      if (brightness == 0) {
        ledEnabled[wavelength] = 0;
      } else {
        ledEnabled[wavelength] = 1;
      }
      break;
    default:
      sendError(ERR_INVALID_ARGS);
      return;
  }

  uint8_t ackPayload[2] = {CMD_SET_LED_STATE, 0x00};
  writeFrame(CMD_TELECOMMAND_ACKNOWLEDGE, ackPayload, sizeof(ackPayload));
}

static void handleFrame(uint8_t frameId, const uint8_t *payload, size_t length) {
  switch (frameId) {
    case 0x00:
      respondPing();
      break;
    case CMD_SYSTEM_STATUS:
      respondSys();
      break;
    case CMD_EXPERIMENT_STATUS:
      respondExperimentStatus();
      break;
    case CMD_HARDWARE_HEALTH_STATUS:
      respondHardwareHealth();
      break;
    case CMD_ENVIRONMENT_INFORMATION:
      respondEnv();
      break;
    case CMD_PHOTOSENSOR_RESULTS:
      respondPhoto();
      break;
    case CMD_SET_LED_STATE:
      respondLed(payload, length);
      break;
    default:
      sendError(ERR_UNKNOWN_FRAME_ID);
      break;
  }
}

static void pollSerial() {
  static enum {
    WAIT_DELIM,
    READ_ID,
    READ_PAYLOAD,
    READ_CHECKSUM
  } state = WAIT_DELIM;
  static uint8_t frameId = 0;
  static uint8_t payload[SERIAL_MAX_PAYLOAD];
  static uint8_t payloadLen = 0;
  static uint8_t expectedLen = 0;

  while (Serial.available() > 0) {
    uint8_t byteIn = (uint8_t)Serial.read();
    switch (state) {
      case WAIT_DELIM:
        if (byteIn == FRAME_DELIM) {
          state = READ_ID;
        }
        break;
      case READ_ID:
        frameId = byteIn;
        expectedLen = expectedPayloadLength(frameId);
        if (expectedLen == 0xFF) {
          sendError(ERR_UNKNOWN_FRAME_ID);
          state = WAIT_DELIM;
        } else if (expectedLen == 0) {
          payloadLen = 0;
          state = READ_CHECKSUM;
        } else {
          payloadLen = 0;
          state = READ_PAYLOAD;
        }
        break;
      case READ_PAYLOAD:
        if (payloadLen < SERIAL_MAX_PAYLOAD) {
          payload[payloadLen++] = byteIn;
        }
        if (payloadLen >= expectedLen) {
          state = READ_CHECKSUM;
        }
        break;
      case READ_CHECKSUM: {
        uint8_t expectedChecksum = checksumFor(frameId, payload, expectedLen);
        if (byteIn == expectedChecksum) {
          handleFrame(frameId, payload, expectedLen);
        }
        state = WAIT_DELIM;
        break;
      }
    }
  }
}

void setup() {
  Serial.begin(115200);
  while (!Serial) {
    delay(10);
  }
  fakeUptimeStart = millis();
}

void loop() {
  pollSerial();
}
