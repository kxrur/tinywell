#include <Wire.h>

// I2C configuration
#define I2C_ADDR 0x48
#define I2C_CLOCK_HZ 100000

// Uncomment and set pins if you need non-default I2C pins on Pico
// #define I2C_SDA_PIN 4
// #define I2C_SCL_PIN 5

// Telecommands
#define CMD_NEXT_EXPERIMENT_PHASE   0x01
#define CMD_RUN_EXPERIMENT_PHASE    0x02
#define CMD_SET_LED_STATE           0x03
#define CMD_TEST_SYSTEM_HEALTH      0x04
#define CMD_SET_STATE_IDLE          0x05
#define CMD_RESET                   0x55

// Telemetry requests
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
#define ERR_I2C_READ_FAILED         0x04
#define ERR_I2C_WRITE_FAILED        0x05
#define ERR_ACK_FAILED              0x06

// LED telecommand arguments
#define LED_ACTION_OFF              0
#define LED_ACTION_ON               1
#define LED_ACTION_TOGGLE           2
#define LED_ACTION_SET_BRIGHTNESS   3

static const uint32_t serialPollIntervalMs = 0;
static uint32_t lastSerialPollMs = 0;

static const size_t SERIAL_MAX_PAYLOAD = 64;

static uint16_t le16(const uint8_t *b) {
  return (uint16_t)b[0] | ((uint16_t)b[1] << 8);
}

static uint32_t le32(const uint8_t *b) {
  return (uint32_t)b[0] | ((uint32_t)b[1] << 8) | ((uint32_t)b[2] << 16) | ((uint32_t)b[3] << 24);
}

static int32_t le32s(const uint8_t *b) {
  return (int32_t)le32(b);
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

static bool readTelemetry(uint8_t cmd, uint8_t *buffer, size_t length) {
  Wire.beginTransmission(I2C_ADDR);
  Wire.write(cmd);
  uint8_t err = Wire.endTransmission(false);
  if (err != 0) {
    return false;
  }

  size_t received = Wire.requestFrom(I2C_ADDR, (uint8_t)length);
  if (received != length) {
    while (Wire.available()) {
      Wire.read();
    }
    return false;
  }

  for (size_t i = 0; i < length; i++) {
    buffer[i] = Wire.read();
  }
  return true;
}

static bool sendTelecommand(const uint8_t *payload, size_t length) {
  Wire.beginTransmission(I2C_ADDR);
  Wire.write(payload, length);
  return Wire.endTransmission(true) == 0;
}

static bool readTelecommandAck(uint8_t *tcId, uint8_t *tcResult) {
  uint8_t buf[2] = {0};
  if (!readTelemetry(CMD_TELECOMMAND_ACKNOWLEDGE, buf, sizeof(buf))) {
    return false;
  }
  *tcId = buf[0];
  *tcResult = buf[1];
  return true;
}

static bool sendEnvironmentInformation() {
  uint8_t buf[14] = {0};
  if (!readTelemetry(CMD_ENVIRONMENT_INFORMATION, buf, sizeof(buf))) {
    return false;
  }

  writeFrame(CMD_ENVIRONMENT_INFORMATION, buf, sizeof(buf));
  return true;
}

static bool sendPhotosensorResults() {
  uint8_t buf[57] = {0};
  if (!readTelemetry(CMD_PHOTOSENSOR_RESULTS, buf, sizeof(buf))) {
    return false;
  }

  writeFrame(CMD_PHOTOSENSOR_RESULTS, buf, sizeof(buf));
  return true;
}

static bool sendSystemStatus() {
  uint8_t buf[5] = {0};
  if (!readTelemetry(CMD_SYSTEM_STATUS, buf, sizeof(buf))) {
    return false;
  }

  writeFrame(CMD_SYSTEM_STATUS, buf, sizeof(buf));
  return true;
}

static bool sendLedCommand(uint8_t wavelength, uint8_t action, uint8_t brightness) {
  uint8_t payload[4] = {CMD_SET_LED_STATE, wavelength, action, brightness};
  if (!sendTelecommand(payload, sizeof(payload))) {
    return false;
  }

  delay(5);
  return true;
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

static void handleFrame(uint8_t frameId, const uint8_t *payload, size_t length) {
  switch (frameId) {
    case 0x00:
      writeFrame(0x00, nullptr, 0);
      break;
    case CMD_SYSTEM_STATUS:
      if (!sendSystemStatus()) {
        sendError(ERR_I2C_READ_FAILED);
      }
      break;
    case CMD_ENVIRONMENT_INFORMATION:
      if (!sendEnvironmentInformation()) {
        sendError(ERR_I2C_READ_FAILED);
      }
      break;
    case CMD_PHOTOSENSOR_RESULTS:
      if (!sendPhotosensorResults()) {
        sendError(ERR_I2C_READ_FAILED);
      }
      break;
    case CMD_SET_LED_STATE: {
      if (length != 3) {
        sendError(ERR_INVALID_LENGTH);
        break;
      }
      uint8_t wavelength = payload[0];
      uint8_t action = payload[1];
      uint8_t brightness = payload[2];
      if (wavelength > 3 || action > LED_ACTION_SET_BRIGHTNESS) {
        sendError(ERR_INVALID_ARGS);
        break;
      }
      if (!sendLedCommand(wavelength, action, brightness)) {
        sendError(ERR_I2C_WRITE_FAILED);
        break;
      }

      uint8_t tcId = 0;
      uint8_t tcResult = 0;
      if (!readTelecommandAck(&tcId, &tcResult)) {
        sendError(ERR_ACK_FAILED);
        break;
      }
      uint8_t ackPayload[2] = {tcId, tcResult};
      writeFrame(CMD_TELECOMMAND_ACKNOWLEDGE, ackPayload, sizeof(ackPayload));
      break;
    }
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

#ifdef I2C_SDA_PIN
  Wire.setSDA(I2C_SDA_PIN);
#endif
#ifdef I2C_SCL_PIN
  Wire.setSCL(I2C_SCL_PIN);
#endif
  Wire.begin();
  Wire.setClock(I2C_CLOCK_HZ);
}

void loop() {
  uint32_t now = millis();
  if (serialPollIntervalMs == 0 || now - lastSerialPollMs >= serialPollIntervalMs) {
    lastSerialPollMs = now;
    pollSerial();
  }
}
