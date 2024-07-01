import mqtt from 'mqtt';
import { ref } from 'vue';

// MQTT connection options
const options = {
    host: '192.168.137.1',
    port: 1883,
    protocol: 'ws',
};

const client = mqtt.connect(options);

// Reactive references to hold the sensor data
const track = ref<number | null>(null);
const sonar = ref<number | null>(null);
const light = ref<number | null>(null);

// Function to initialize MQTT connection and subscriptions
const initializeMQTT = () => {
    client.on('connect', () => {
        console.log('Connected to MQTT broker');
        client.subscribe('esp32/track');
        client.subscribe('esp32/sonar');
        client.subscribe('esp32/light');
    });

    client.on('message', (topic, message) => {
        const data = message.toString();
        switch (topic) {
            case 'esp32/track':
                track.value = parseFloat(data);
                break;
            case 'esp32/sonar':
                sonar.value = parseFloat(data);
                break;
            case 'esp32/light':
                light.value = parseFloat(data);
                break;
        }
    });

    client.on('error', (error) => {
        console.error('MQTT Connection Error:', error);
    });
};

// Export the reactive references and initialization function
export const useMQTT = () => {
    return {
        track,
        sonar,
        light,
        initializeMQTT,
    };
};
