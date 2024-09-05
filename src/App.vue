<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { initializeWebSocket } from './utils/websocket';
import {
  initializeControllerEvents,
  x,
  y,
  numberArrayToSend,
  // currentFace, <- sadly did not came with it
  headRotation,
  videoOn,
  buzzerOn,
  //ledAnimation, <- because of the reading of the battery which is on the same pin we can't change the led animation
} from './utils/controllerEvents';

const wsTestingServer = 'ws://localhost:8080';
const carIP = ref('172.30.128.50'); // Add the IP of the car here
const carCameraPort = '7000';
const carWebSocket = ref(`ws://${carIP.value}/carwebsocket`);

const switchToTestingServer = () => {
  carWebSocket.value = wsTestingServer;
};

const restartWS = () => {
  initializeWebSocket(carWebSocket.value);
};

onMounted(() => {
  initializeWebSocket(carWebSocket.value);
  initializeControllerEvents();
});

function getDirection(x: number, y: number): string {
  if (y === 0 && x > 0) return '⬆️';
  if (y === 0 && x < 0) return '⬇️';
  if (x === 0 && y > 0) return '➡️';
  if (x === 0 && y < 0) return '⬅️';
  if (x > 0 && y > 0) return '↗️';
  if (x > 0 && y < 0) return '↖️';
  if (x < 0 && y > 0) return '↙️';
  if (x < 0 && y < 0) return '↘️';

  return 'Car is stopped 🛑';
}

function getHeadRotation(rotation: [number, number] | number[]): string {
  const [horizontal, vertical] = Array.isArray(rotation) ? rotation : [rotation, rotation];

  if (horizontal === 90 && vertical === 90) return 'Looking straight forward';

  let direction = '';

  // Horizontal rotation
  if (horizontal > 90) direction += 'Right ';
  else if (horizontal < 90) direction += 'Left ';

  // Vertical rotation
  if (vertical > 90) direction += 'Up';
  else if (vertical < 90) direction += 'Down';

  // If no direction (horizontal = 90 and vertical != 90)
  if (!direction) direction = 'Looking straight';

  return direction.trim();
}

</script>

<template>
  <!-- <main>
    <input v-model="carIP" placeholder="Car IP" />
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

      <h1>Head Rotation</h1>
      <p>Head rotation: {{ headRotation }}</p>

      <span>
        Buzzer: 🚨 {{ buzzerOn ? 'On' : 'Off' }}
      </span>
      <span>
        Led Animation number: {{ ledAnimation }}
      </span>
    </div>
  </main> -->
  <main>
    <section>
      <div class="camera_preview_wrapper">
        <img v-if="videoOn" :src="`http://${carIP}:${carCameraPort}`" alt="Car camera" ref="carCamera"
          class="camera_preview" />
        <img v-else src="./assets/no_signal.jpg" alt="No camera" class="camera_preview" height="500" width="500" />
        <span class="camera_preview_ip">{{ carIP.length > 0 ? `${carIP}:${carCameraPort}` : 'No IP'
          }}</span>
      </div>

      <div class="car_controller">
        <div class="directions">
          <span>
            Wheels: {{ numberArrayToSend }}
          </span>
          <span>
            Directions: {{ getDirection(x, y) }}
          </span>
        </div>
        <div class="face_rotation">
          <span>The head is {{ getHeadRotation(headRotation) }}</span>
        </div>
        <div class="buzzer">
          <span>Buzzer: 🚨 {{ buzzerOn ? 'On' : 'Off' }}</span>
        </div>
      </div>
    </section>
  </main>
</template>



<style scoped lang="scss">
section {
  height: calc(100vh - 16px);
  width: 100%;
  display: grid;
  place-items: center;

  grid-template-columns: repeat(16, 1fr);
  grid-template-rows: repeat(9, 1fr);
}

.camera_preview_wrapper {
  position: relative;

  grid-column: 1 / 5;
  grid-row: 1 / 5;


}

.camera_preview {
  border-radius: 1rem;
}

.camera_preview_ip {
  position: absolute;
  top: 1.5rem;
  left: 0.5rem;
  background-color: rgba(0, 0, 0, 0.5);
  color: white;
  padding: 4px;
}

span {
  font-size: 1.5rem;
}

.car_controller {
  grid-column: 3 / 16;
  grid-row: 1 / 9;
  display: flex;
  flex-direction: column;
  justify-content: space-around;
}
</style>
