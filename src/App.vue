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
    <span v-if="videoOn">🎥: Video is on</span>
    <span v-else>🚫: Video is off</span>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref, watch } from 'vue';
import { listen } from '@tauri-apps/api/event';
import { calculateWheelDirections } from './calculateWheelDirections';

// WebSocket setup
const wsTestingServer = 'ws://localhost:8080';
const carWebSocket = 'ws://192.168.109.50:7000/carwebsocket';
const websocketAddress = carWebSocket;
let ws = new WebSocket(websocketAddress);

ws.onopen = () => {
  console.log('WebSocket connection established');
};

ws.onmessage = (message) => {
  console.log('Received message:', message.data);
};

ws.onclose = () => {
  console.log('WebSocket connection closed');
};

ws.onerror = (error) => {
  console.error('WebSocket error:', error);
};

// Controller values
const x = ref(0);
const y = ref(0);
const numberArrayToSend = ref([0, 0, 0, 0]);
// Face value
const currentFace = ref(0);
const receivedFace = ref(0);
// head rotation array
const headRotation = ref([0, 0]);
const receivedHeadRotation = ref([0, 0]);
// Video On/Off
const videoOn = ref(0);
const receivedVideoOn = ref(0);

onMounted(() => {
  listen('controller', (event) => {
    x.value = event.payload.x;
    y.value = event.payload.y;
    receivedFace.value = event.payload.face;
    receivedHeadRotation.value = event.payload.headRotation;
    receivedVideoOn.value = event.payload.videoOn;
  });
});

// Watch x and y to update numberArrayToSend and send via WebSocket
watch([x, y], ([newX, newY]) => {
  // Controller values
  numberArrayToSend.value = calculateWheelDirections(newX, newY);
  sendMessage(1, numberArrayToSend.value);
});

// When we receive a face value, update the currentFace value
watch(receivedFace, (newFace) => {
  currentFace.value = newFace;
  sendMessage(2, newFace);
});

// When we receive a head rotation value, update the headRotation value
watch(receivedHeadRotation, (newHeadRotation) => {
  headRotation.value = newHeadRotation;
  if (newHeadRotation[0] !== 0 || newHeadRotation[1] !== 0) {
    sendMessage(3, newHeadRotation);
  }
});

// When we receive a videoOn value, update the videoOn value
watch(receivedVideoOn, (newVideoOn) => {
  videoOn.value = newVideoOn;
  // on: 1, off: 0
  sendMessage(9, newVideoOn ? 1 : 0);
});

const sendMessage = (type: number, data: unknown[] | number) => {
  // if type is not between 1 and 9, return
  if (type < 1 || type > 9) {
    console.error('Invalid type:', type);
    return;
  }
  if (ws && ws.readyState === WebSocket.OPEN) {
    const message = {
      cmd: type,
      data: data,
    };
    ws.send(JSON.stringify(message));
    console.log('Message sent:', message);
  } else {
    console.error('WebSocket is not open');
  }
};
</script>

<style scoped>
h1 {
  font-size: 2em;
  margin-bottom: 0.5em;
}
</style>