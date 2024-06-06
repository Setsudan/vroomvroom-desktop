// controllerEvents.ts
import { ref, watch } from 'vue';
import { listen } from '@tauri-apps/api/event';
import { calculateWheelDirections } from './calculateWheelDirections';
import { sendMessage } from './websocket';

export const x = ref(0);
export const y = ref(0);
export const numberArrayToSend = ref([0, 0, 0, 0]);
export const currentFace = ref(0);
export const receivedFace = ref(0);
export const headRotation = ref([0, 0]);
export const receivedHeadRotation = ref([0, 0]);
export const videoOn = ref(0);
export const receivedVideoOn = ref(0);
export const buzzerOn = ref(0);
export const ledAnimation = ref(0);
// La première valeur est la représentation binaire d'un bitmask identifiant la LED, les 3 valeurs suivantes sont des valeurs RGB entre 0 et 255
export const ledColors = ref([0, 0, 0, 0]);

export const initializeControllerEvents = () => {
    listen('controller', (event) => {
        x.value = event.payload.x;
        y.value = event.payload.y;
        receivedFace.value = event.payload.face;
        receivedHeadRotation.value = event.payload.headRotation;
        receivedVideoOn.value = event.payload.videoOn;
        buzzerOn.value = event.payload.buzzerOn;
        ledAnimation.value = event.payload.ledAnimation;
        ledColors.value = event.payload.ledColors;
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
        headRotation.value = newHeadRotation;
        sendMessage(3, newHeadRotation);
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
        // we wanna toggle it on and off realquick
        //like 300ms
        // sendMessage(7, newBuzzerOn);
        const timer = setInterval(() => {
            sendMessage(7, newBuzzerOn);
            clearInterval(timer);
        }, 300);
        // once timer is done, we send the opposite value
        setTimeout(() => {
            sendMessage(7, newBuzzerOn ? 0 : 1);
        }, 300);
    });

    watch(receivedVideoOn, (newVideoOn) => {
        videoOn.value = newVideoOn;
        sendMessage(9, newVideoOn ? 1 : 0);
    });
};