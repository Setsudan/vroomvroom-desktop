<template>
  <div>
    <h1>Controller Values</h1>
    <!-- X is the RB(forward) and LB(backward) values -->
    <p>X: {{ x }}</p>
    <!-- Y is the LeftStick values -->
    <p>Y: {{ y }}</p>
    <span>
      <!-- Thoses are the numbers we send to the car it's an array of 4 numbers -->
      Array for the wheels: {{ numberArrayToSend }}
    </span>
    <span class="direction">
      <!-- If y is 0 but X is positive we go forward if X is negative but y is 0 we go backward
      if x is different than 0 and y is positive we go right if x is different than 0 and y is negative we go left
      -->
      <span v-if="y === 0 && x > 0">⬆️</span>
      <span v-else-if="y === 0 && x < 0">⬇️</span>
      <span v-else-if="x === 0 && y > 0">➡️</span>
      <span v-else-if="x === 0 && y < 0">⬅️</span>
      <!-- If x is positive and y is positive we go diagonaly right -->
      <span v-else-if="x > 0 && y > 0">↗️</span>
      <!-- If x is positive and y is negative we go diagonaly left -->
      <span v-else-if="x > 0 && y < 0">↖️</span>
      <!-- If x is negative and y is positive we go diagonaly left -->
      <span v-else-if="x < 0 && y > 0">↙️</span>
      <!-- If x is negative and y is negative we go diagonaly right -->
      <span v-else-if="x < 0 && y < 0">↘️</span>
      <span v-else>
        <!-- Car is stopped -->
        🛑
      </span>
    </span>

    <h1>Face Value</h1>
    <!-- from 0 to 7 -->
    <p>Current face: {{ currentFace }}</p>

    <h1>Head Rotation</h1>
    <!-- Array of 2 number of for the vertical rotation the second for the horizontal -->
    <p>Head rotation: {{ headRotation }}</p>

    <h1>Video On/Off</h1>
    <div v-if="videoOn"><span>🎥: Video is on</span>
      <!-- If video is on the video influx is availiable on carIp + ':' + carCameraPort -->
      <div>Video influx: {{ carIP + ':' + carCameraPort }}</div>
      <!-- Preview of camera -->
      <img :src="'http://' + carIP + ':' + carCameraPort" alt="Car camera" />
    </div>
    <span v-else>🚫: Video is off</span>

    <span>
      Buzzer: 🚨
    </span>
    <span>
      {{ buzzerOn ? 'On' : 'Off' }}
    </span>
    <span>
      Led Animation number: {{ ledAnimation }}
    </span>

    <h1>Mqtt Data</h1>
    <div>
    <p>Battery Voltage: {{ sensorData.battery_voltage }}</p>
    <p>Photosensitive Value: {{ sensorData.photosensitive_value }}</p>
    <p>Tracking Module Values: {{ sensorData.tracking_module_values }}</p>
    <p>Ultrasonic Distance: {{ sensorData.ultrasonic_distance }}</p>
    <p>Motor Speeds: {{ sensorData.motor_speeds }}</p>
    <p>Buzzer Status: {{ sensorData.buzzer_status }}</p>
    <p>Buzzer Frequency: {{ sensorData.buzzer_frequency }}</p>
  </div>

  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue';
import { initializeWebSocket } from './utils/websocket';
import { initializeControllerEvents, x, y, numberArrayToSend, currentFace, headRotation, videoOn, buzzerOn, ledAnimation } from './utils/controllerEvents';
import { listen } from '@tauri-apps/api/event';

const wsTestingServer = 'ws://localhost:8080';
const carIP = '192.168.109.50';
const carCameraPort = '7000';
const carWebSocket = 'ws://192.168.109.50/carwebsocket';

const sensorData = ref({
  battery_voltage: 0,
  photosensitive_value: 0,
  tracking_module_values: 0,
  ultrasonic_distance: 0,
  motor_speeds: [0, 0, 0, 0],
  buzzer_status: '',
  buzzer_frequency: 0,
});

onMounted(() => {
  initializeWebSocket(carWebSocket);
  initializeControllerEvents();
  listen('sensor-data', (event) => {
    sensorData.value = event.payload;
  });
});
</script>

<style scoped>
h1 {
  font-size: 2em;
  margin-bottom: 0.5em;
}
</style>