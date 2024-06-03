<template>
  <div>
    <h1>Controller Values</h1>
    <p>X: {{ x }}</p>
    <p>Y: {{ y }}</p>
    <span>
      Array for the wheels: {{ numberArrayToSend }}
    </span>

    <h1>Face Value</h1>
    <p>Current face: {{ currentFace }}</p>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref, watch } from 'vue';
import { listen } from '@tauri-apps/api/event';
import { calculateWheelDirections } from './calculateWheelDirections';

// WebSocket setup
const wsTestingServer = 'ws://localhost:8080';
//const carWebSocket = 'ws://192.168.123.220/carwebsocket';
const websocketAddress = wsTestingServer;
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

onMounted(() => {
  listen('controller', (event) => {
    x.value = event.payload.x;
    y.value = event.payload.y;
    receivedFace.value = event.payload.face;
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
