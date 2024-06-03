import { WebSocket } from 'ws';
//const CAR_IP_ADDRESS = '192.168.145.49';  // IP address of the car
//const WEBSOCKET_URL = `ws://${CAR_IP_ADDRESS}/ws`;

const WEBSOCKET_URL = 'ws://localhost:8080';  // Local server URL

let ws: WebSocket | null = null;

export function connectWebSocket() {
  ws = new WebSocket(WEBSOCKET_URL);

  ws.onopen = () => {
    console.log('WebSocket connection opened.');
  };

  ws.onclose = () => {
    console.log('WebSocket connection closed. Attempting to reconnect...');
    setTimeout(connectWebSocket, 5000);  // Try to reconnect every 5 seconds
  };

  ws.onerror = (error) => {
    console.error('WebSocket error:', error);
  };

  ws.onmessage = (event) => {
    console.log('Received message from server:', event.data);
  };
}

export function sendWheelSpeeds(speeds: number[]) {
  if (ws && ws.readyState === WebSocket.OPEN) {
    const message = JSON.stringify({
      cmd: 1,
      data: speeds
    });
    console.log('Sending message:', message);
    ws.send(message);
  } else {
    console.warn('WebSocket is not connected.');
  }
}

