<template>
  <div>
    <h1>Controller Values</h1>
    <p>X: {{ x }}</p>
    <p>Y: {{ y }}</p>
    <span>
      Array for the wheels: {{ numberArrayToSend }}
    </span>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref, watch } from 'vue';
import { listen } from '@tauri-apps/api/event';
import { calculateWheelDirections } from './calculateWheelDirections';

// WebSocket setup
const websocketAddress = 'ws://localhost:8080';
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

const x = ref(0);
const y = ref(0);
const numberArrayToSend = ref([0, 0, 0, 0]);

onMounted(() => {
  listen('controller', (event) => {
    x.value = event.payload.x;
    y.value = event.payload.y;
  });
});

// Watch x and y to update numberArrayToSend and send via WebSocket
watch([x, y], ([newX, newY]) => {
  numberArrayToSend.value = calculateWheelDirections(newX, newY);
  sendMessage();
});

const sendMessage = () => {
  if (ws && ws.readyState === WebSocket.OPEN) {
    const message = {
      cmd: 1,
      data: numberArrayToSend.value,
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
