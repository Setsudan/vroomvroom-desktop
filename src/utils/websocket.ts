// websocket.ts
let ws: WebSocket;

export const initializeWebSocket = (url: string) => {
  ws = new WebSocket(url);

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
};

export const sendMessage = (type: number, data: unknown[] | number) => {
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

export const sendMessagePy = (type: number, data: unknown) => {
  if (type < 1 || type > 9) {
    console.error('Invalid type:', type);
    return;
  }
  if (ws && ws.readyState === WebSocket.OPEN) {
    const message = {
      cmd: type,
      data: {
        horn: data,
        frequency: 300
      },
    };
    ws.send(JSON.stringify(message));
    console.log('Message sent:', message);
  } else {
    console.error('WebSocket is not open');
  }
}
