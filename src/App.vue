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
      <img :src="`http://${carIP}:${carCameraPort}`" alt="Car camera" ref="carCamera" />
      <button @click="startRecording">Start Recording</button>
      <button @click="stopRecording">Stop Recording</button>
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
import { createFFmpeg, fetchFile } from '@ffmpeg/ffmpeg';

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


const carCamera = ref<HTMLImageElement | null>(null);
const recording = ref(false);
const frames: string[] = [];
let intervalId: number | null = null;
let ffmpeg: any = null;

const loadFFmpeg = async () => {
  ffmpeg = createFFmpeg({ log: true });
  await ffmpeg.load();
};

const startRecording = () => {
  recording.value = true;
  frames.length = 0; // Clear previous frames
  intervalId = setInterval(captureFrame, 100); // Capture frame every 100ms
};

const stopRecording = () => {
  recording.value = false;
  if (intervalId !== null) clearInterval(intervalId);
  createVideo();
};

const captureFrame = () => {
  if (!recording.value || !carCamera.value) return;

  const imgElement = carCamera.value;
  const canvas = document.createElement('canvas');
  canvas.width = imgElement.width;
  canvas.height = imgElement.height;
  const context = canvas.getContext('2d');
  if (context) {
    context.drawImage(imgElement, 0, 0, canvas.width, canvas.height);
    frames.push(canvas.toDataURL('image/jpeg')); // Store the image data
  }
};

const createVideo = async () => {
  if (!ffmpeg) await loadFFmpeg();

  for (let i = 0; i < frames.length; i++) {
    const frame = frames[i];
    const response = await fetch(frame);
    const blob = await response.blob();
    const arrayBuffer = await blob.arrayBuffer();
    ffmpeg.FS('writeFile', `frame${i}.jpg`, new Uint8Array(arrayBuffer));
  }

  await ffmpeg.run(
    '-framerate', '10',
    '-i', 'frame%d.jpg',
    '-c:v', 'libx264',
    '-pix_fmt', 'yuv420p',
    'output.mp4'
  );

  const data = ffmpeg.FS('readFile', 'output.mp4');
  const videoBlob = new Blob([data.buffer], { type: 'video/mp4' });
  const url = URL.createObjectURL(videoBlob);
  const a = document.createElement('a');
  a.href = url;
  a.download = 'recording.mp4';
  a.click();

  // Clean up the virtual file system
  ffmpeg.FS('unlink', 'output.mp4');
  frames.forEach((_, i) => ffmpeg.FS('unlink', `frame${i}.jpg`));
};

onMounted(() => {
  loadFFmpeg();
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