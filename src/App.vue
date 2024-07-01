<template>
  <main>
    <button @click="restartWS">Restart WebSocket</button>
    <div>
      <h1>Controller Values</h1>
      <!-- X is the RB(forward) and LB(backward) values -->
      <p>X: {{ x }}</p>
      <!-- Y is the LeftStick values -->
      <p>Y: {{ y }}</p>
      <span>
        These are the numbers we send to the car. It's an array of 4 numbers.
        Array for the wheels: {{ numberArrayToSend }}
      </span>
      <span class="direction">
        <!-- Direction indicators based on x and y values -->
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
        <p>Battery Voltage: {{ sensorData.battery_voltage }}</p>
        <p>Photosensitive Value: {{ sensorData.photosensitive_value }}</p>
        <p>Tracking Module Values: {{ sensorData.tracking_module_values }}</p>
        <p>Ultrasonic Distance: {{ sensorData.ultrasonic_distance }}</p>
        <p>Motor Speeds: {{ sensorData.motor_speeds.join(', ') }}</p>
        <p>Buzzer Status: {{ sensorData.buzzer_status }}</p>
        <p>Buzzer Frequency: {{ sensorData.buzzer_frequency }}</p>
      </div>
    </div>
  </main>
</template>

<script setup lang="ts">
import { onMounted } from 'vue';
import { initializeWebSocket } from './utils/websocket';
import { initializeControllerEvents, x, y, numberArrayToSend, currentFace, headRotation, videoOn, buzzerOn, ledAnimation, sensorData } from './utils/controllerEvents';

const wsTestingServer = 'ws://localhost:8080';
const carIP = '192.168.31.50';
const carCameraPort = '7000';
const carWebSocket = `ws://${carIP}/carwebsocket`;

const restartWS = () => {
  initializeWebSocket(carWebSocket);
};

onMounted(() => {
  initializeWebSocket(carWebSocket);
  initializeControllerEvents();
});
</script>

<style scoped lang="scss">
h1 {
  font-size: 2em;
  margin-bottom: 0.5em;
}
</style>
