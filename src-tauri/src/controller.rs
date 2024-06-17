use gilrs::{Axis, Button, Event, EventType, Gilrs};
use serde_json::json;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tauri::Manager;

pub const MAX_ROTATION: i32 = 180;
pub const MIN_ROTATION: i32 = 0;
pub const ROTATION_STEP: i32 = 30;
pub const SPEED_SCALE: f64 = 100.0;

pub fn handle_controller_events(
    x: Arc<Mutex<i32>>,
    y: Arc<Mutex<i32>>,
    face: Arc<Mutex<i32>>,
    head_horizontal_rotation: Arc<Mutex<i32>>,
    head_vertical_rotation: Arc<Mutex<i32>>,
    video_on: Arc<AtomicBool>,
    buzzer_on: Arc<AtomicBool>,
    led_animation: Arc<Mutex<i32>>,
) {
    let mut gilrs = Gilrs::new().unwrap();

    thread::spawn(move || loop {
        while let Some(event) = gilrs.next_event() {
            handle_event(
                event,
                &x,
                &y,
                &face,
                &head_horizontal_rotation,
                &head_vertical_rotation,
                &video_on,
                &buzzer_on,
                &led_animation,
            );
        }
        thread::sleep(Duration::from_millis(10));
    });
}

fn handle_event(
    event: Event,
    x: &Arc<Mutex<i32>>,
    y: &Arc<Mutex<i32>>,
    face: &Arc<Mutex<i32>>,
    head_horizontal_rotation: &Arc<Mutex<i32>>,
    head_vertical_rotation: &Arc<Mutex<i32>>,
    video_on: &Arc<AtomicBool>,
    buzzer_on: &Arc<AtomicBool>,
    led_animation: &Arc<Mutex<i32>>,
) {
    match event.event {
        EventType::ButtonChanged(Button::RightTrigger2, value, ..) => {
            let mut x = x.lock().expect("Failed to lock x");
            *x = (value as f64 * SPEED_SCALE) as i32;
        }
        EventType::ButtonChanged(Button::LeftTrigger2, value, ..) => {
            let mut x = x.lock().expect("Failed to lock x");
            *x = (-value as f64 * SPEED_SCALE) as i32;
        }
        EventType::AxisChanged(Axis::LeftStickX, value, ..) => {
            let mut y = y.lock().expect("Failed to lock y");
            *y = (value as f64 * SPEED_SCALE) as i32;
        }
        EventType::ButtonChanged(Button::DPadRight, value, ..) => {
            if value == 1.0 {
                let mut head_horizontal_rotation = head_horizontal_rotation
                    .lock()
                    .expect("Failed to lock head_horizontal_rotation");
                if *head_horizontal_rotation < MAX_ROTATION {
                    *head_horizontal_rotation += ROTATION_STEP;
                }
            }
        }
        EventType::ButtonChanged(Button::DPadLeft, value, ..) => {
            if value == 1.0 {
                let mut head_horizontal_rotation = head_horizontal_rotation
                    .lock()
                    .expect("Failed to lock head_horizontal_rotation");
                if *head_horizontal_rotation > MIN_ROTATION {
                    *head_horizontal_rotation -= ROTATION_STEP;
                }
            }
        }
        EventType::ButtonChanged(Button::DPadUp, value, ..) => {
            if value == 1.0 {
                let mut head_vertical_rotation = head_vertical_rotation
                    .lock()
                    .expect("Failed to lock head_vertical_rotation");
                if *head_vertical_rotation < MAX_ROTATION {
                    *head_vertical_rotation += ROTATION_STEP;
                }
            }
        }
        EventType::ButtonChanged(Button::DPadDown, value, ..) => {
            if value == 1.0 {
                let mut head_vertical_rotation = head_vertical_rotation
                    .lock()
                    .expect("Failed to lock head_vertical_rotation");
                if (*head_vertical_rotation) > MIN_ROTATION {
                    *head_vertical_rotation -= ROTATION_STEP;
                }
            }
        }
        EventType::ButtonChanged(Button::West, value, ..) => {
            if value == 1.0 {
                let mut face = face.lock().expect("Failed to lock face");
                *face = (*face + 1) % 8;
            }
        }
        EventType::ButtonChanged(Button::East, value, ..) => {
            if value == 1.0 {
                let mut face = face.lock().expect("Failed to lock face");
                *face = (*face + 7) % 8;
            }
        }
        EventType::ButtonChanged(Button::Start, value, ..) => {
            if value == 1.0 {
                video_on.fetch_xor(true, Ordering::SeqCst);
            }
        }
        EventType::ButtonChanged(Button::South, value, ..) => {
            if value == 1.0 {
                buzzer_on.fetch_xor(true, Ordering::SeqCst);
            }
        }
        EventType::ButtonChanged(Button::North, value, ..) => {
            if value == 1.0 {
                let mut led_animation = led_animation.lock().expect("Failed to lock led_animation");
                *led_animation = (*led_animation + 1) % 6;
            }
        }
        _ => {}
    }
}

pub fn emit_controller_state(
    app_handle: &tauri::AppHandle,
    x: &Arc<Mutex<i32>>,
    y: &Arc<Mutex<i32>>,
    face: &Arc<Mutex<i32>>,
    head_horizontal_rotation: &Arc<Mutex<i32>>,
    head_vertical_rotation: &Arc<Mutex<i32>>,
    video_on: &Arc<AtomicBool>,
    buzzer_on: &Arc<AtomicBool>,
    led_animation: &Arc<Mutex<i32>>,
) {
    let x = *x.lock().expect("Failed to lock x");
    let y = *y.lock().expect("Failed to lock y");
    let face = *face.lock().expect("Failed to lock face");
    let head_horizontal_rotation = *head_horizontal_rotation
        .lock()
        .expect("Failed to lock head_horizontal_rotation");
    let head_vertical_rotation = *head_vertical_rotation
        .lock()
        .expect("Failed to lock head_vertical_rotation");
    let head_rotation_array = vec![head_horizontal_rotation, head_vertical_rotation];
    let video_on = video_on.load(Ordering::SeqCst);
    let buzzer_on = buzzer_on.load(Ordering::SeqCst);
    let led_animation = *led_animation.lock().expect("Failed to lock led_animation");

    app_handle
        .emit_all(
            "controller",
            json!({
                "x": x,
                "y": y,
                "face": face,
                "headRotation": head_rotation_array,
                "videoOn": video_on,
                "buzzerOn": buzzer_on,
                "ledAnimation": led_animation,
            }),
        )
        .expect("Failed to emit controller state");
}
