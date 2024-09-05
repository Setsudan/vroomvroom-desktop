import { ref, watch } from 'vue';
import { listen } from '@tauri-apps/api/event';
import { calculateWheelDirections } from './calculateWheelDirections';
import { sendMessage as sendOriginalMessage } from './websocket';

export const x = ref(0);
export const y = ref(0);
export const numberArrayToSend = ref([0, 0, 0, 0]);
export const currentFace = ref(0);
export const receivedFace = ref(0);
export const headRotation = ref([90, 90]);
export const receivedHeadRotation = ref([0, 0]);
export const videoOn = ref(0);
export const receivedVideoOn = ref(0);
export const buzzerOn = ref(0);
export const ledAnimation = ref(0);
// La première valeur est la représentation binaire d'un bitmask identifiant la LED, les 3 valeurs suivantes sont des valeurs RGB entre 0 et 255
export const ledColors = ref([0, 0, 0, 0]);

const rotationIncrement = 30; // Define the required increment

const hasSignificantChange = (newValues, oldValues, increment) => {
    return newValues.some((newValue, index) => Math.abs(newValue - oldValues[index]) >= increment);
};

// Message queue
const messageQueue = [];
let isSending = false;

// Rate limiting function
const processQueue = () => {
    if (messageQueue.length === 0) {
        isSending = false;
        return;
    }
    isSending = true;
    const { type, payload } = messageQueue.shift();
    sendOriginalMessage(type, payload);
    setTimeout(processQueue, 27); // Wait 50ms before sending the next message
};

// Add messages to the queue
const sendMessage = (type, payload) => {
    messageQueue.push({ type, payload });
    if (!isSending) {
        processQueue();
    }
};

export const initializeControllerEvents = () => {
    listen('controller', (event: unknown) => {
        const payload = event.payload as any;
        x.value = payload.x;
        y.value = payload.y;
        receivedFace.value = payload.face;
        receivedHeadRotation.value = payload.headRotation;
        receivedVideoOn.value = payload.videoOn;
        buzzerOn.value = payload.buzzerOn;
        ledAnimation.value = payload.ledAnimation;
        ledColors.value = payload.ledColors;
    });

    watch([x, y], ([newX, newY]) => {
        numberArrayToSend.value = calculateWheelDirections(newX, newY);
        sendMessage(1, numberArrayToSend.value);
    });

    watch(receivedFace, (newFace) => {
        currentFace.value = newFace;
        sendMessage(2, newFace);
    });

    watch(receivedHeadRotation, (newHeadRotation) => {
        if (hasSignificantChange(newHeadRotation, headRotation.value, rotationIncrement)) {
            headRotation.value = newHeadRotation;
            sendMessage(3, newHeadRotation);
        }
    });

    watch(ledAnimation, (newLedAnimation) => {
        ledAnimation.value = newLedAnimation;
        sendMessage(4, newLedAnimation);
    });

    watch(ledColors, (newledColors) => {
        ledColors.value = newledColors;
        sendMessage(5, newledColors);
    });

    watch(buzzerOn, (newBuzzerOn) => {
        sendMessage(7, newBuzzerOn ? 1 : 0);
    });

    watch(receivedVideoOn, (newVideoOn) => {
        videoOn.value = newVideoOn;
        sendMessage(9, newVideoOn ? 1 : 0);
    });
};
