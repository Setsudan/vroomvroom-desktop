<template>
  <main>
    <button @click="restartWS">Restart WebSocket</button>
    <button @click="switchToTestingServer">Switch to Testing Server</button>
    <div>
      <h1>Controller Values</h1>
      <p>X: {{ x }}</p>
      <p>Y: {{ y }}</p>
      <span>
        These are the numbers we send to the car. It's an array of 4 numbers.
        Array for the wheels: {{ numberArrayToSend }}
      </span>
      <span class="direction">
        <span v-if="y === 0 && x > 0">⬆️</span>
        <span v-else-if="y === 0 && x < 0">⬇️</span>
        <span v-else-if="x === 0 && y > 0">➡️</span>
        <span v-else-if="x === 0 && y < 0">⬅️</span>
        <span v-else-if="x > 0 && y > 0">↗️</span>
        <span v-else-if="x > 0 && y < 0">↖️</span>
        <span v-else-if="x < 0 && y > 0">↙️</span>
        <span v-else-if="x < 0 && y < 0">↘️</span>
        <span v-else>
          Car is stopped
          🛑
        </span>
      </span>

      <h1>Face Value</h1>
      <p>Current face: {{ currentFace }}</p>

      <h1>Head Rotation</h1>
      <p>Head rotation: {{ headRotation }}</p>

      <h1>Video On/Off</h1>
      <div v-if="videoOn">
        <span>🎥: Video is on</span>
        <div>Video influx: {{ carIP + ':' + carCameraPort }}</div>
        Preview of camera
        <img :src="`http://${carIP}:${carCameraPort}`" alt="Car camera" ref="carCamera" />
      </div>
      <span v-else>🚫: Video is off</span>

      <span>
        Buzzer: 🚨 {{ buzzerOn ? 'On' : 'Off' }}
      </span>
      <span>
        Led Animation number: {{ ledAnimation }}
      </span>

      <h1>Mqtt Data</h1>
      <div v-if="sensorData">
        <p>Track: {{ sensorData.track }}</p>
        <p>Ultrasonic Distance: {{ sensorData.sonar }}</p>
        <p>Photosensitive Value: {{ sensorData.light }}</p>
      </div>
    </div>
  </main>
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from 'vue';
import { useMQTT } from './mqttService';
import { initializeWebSocket } from './utils/websocket';
import {
  initializeControllerEvents,
  x,
  y,
  numberArrayToSend,
  currentFace,
  headRotation,
  videoOn,
  buzzerOn,
  ledAnimation,
} from './utils/controllerEvents';

const { track, sonar, light, initializeMQTT } = useMQTT();

const sensorData = ref({
  track: track.value,
  sonar: sonar.value,
  light: light.value,
});

watch([track, sonar, light], () => {
  sensorData.value.track = track.value;
  sensorData.value.sonar = sonar.value;
  sensorData.value.light = light.value;
});

const wsTestingServer = 'ws://localhost:8080';
const carIP = '192.168.137.50';
const carCameraPort = '7000';
const carWebSocket = ref(`ws://${carIP}/carwebsocket`);

const switchToTestingServer = () => {
  carWebSocket.value = wsTestingServer;
};

const restartWS = () => {
  initializeWebSocket(carWebSocket.value);
};

onMounted(() => {
  initializeWebSocket(carWebSocket.value);
  initializeControllerEvents();
  initializeMQTT();
});
</script>

<style scoped lang="scss">
h1 {
  font-size: 2em;
  margin-bottom: 0.5em;
}
</style>
