#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use gilrs::{Axis, Button, EventType, Gilrs};
use serde_json::json;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;
use tauri::Manager;

const MAX_ROTATION: i32 = 180;
const MIN_ROTATION: i32 = 0;
const ROTATION_STEP: i32 = 30;
const SPEED_SCALE: f64 = 100.0;

fn main() {
    let mut gilrs = Gilrs::new().unwrap();

    let x = Arc::new(Mutex::new(0));
    let y = Arc::new(Mutex::new(0));
    let face = Arc::new(Mutex::new(0));
    let head_horizontal_rotation = Arc::new(Mutex::new(90));
    let head_vertical_rotation = Arc::new(Mutex::new(90));
    let video_on = Arc::new(AtomicBool::new(false));
    let buzzer_on = Arc::new(AtomicBool::new(false));
    let led_animation = Arc::new(Mutex::new(0));

    let x_clone = Arc::clone(&x);
    let y_clone = Arc::clone(&y);
    let face_clone = Arc::clone(&face);
    let head_horizontal_rotation_clone = Arc::clone(&head_horizontal_rotation);
    let head_vertical_rotation_clone = Arc::clone(&head_vertical_rotation);
    let video_on_clone = Arc::clone(&video_on);
    let buzzer_on_clone = Arc::clone(&buzzer_on);
    let led_animation_clone = Arc::clone(&led_animation);

    thread::spawn(move || {
        loop {
            while let Some(event) = gilrs.next_event() {
                handle_event(event, &x_clone, &y_clone, &face_clone, &head_horizontal_rotation_clone, &head_vertical_rotation_clone, &video_on_clone, &buzzer_on_clone, &led_animation_clone);
            }
            thread::sleep(Duration::from_millis(10));
        }
    });

    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle();
            let x = Arc::clone(&x);
            let y = Arc::clone(&y);
            let face = Arc::clone(&face);
            let head_horizontal_rotation = Arc::clone(&head_horizontal_rotation);
            let head_vertical_rotation = Arc::clone(&head_vertical_rotation);
            let video_on = Arc::clone(&video_on);
            let buzzer_on = Arc::clone(&buzzer_on);
            let led_animation = Arc::clone(&led_animation);

            thread::spawn(move || loop {
                emit_controller_state(&app_handle, &x, &y, &face, &head_horizontal_rotation, &head_vertical_rotation, &video_on, &buzzer_on, &led_animation);
                thread::sleep(Duration::from_millis(100));
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn handle_event(event: gilrs::Event, x: &Arc<Mutex<i32>>, y: &Arc<Mutex<i32>>, face: &Arc<Mutex<i32>>, head_horizontal_rotation: &Arc<Mutex<i32>>, head_vertical_rotation: &Arc<Mutex<i32>>, video_on: &Arc<AtomicBool>, buzzer_on: &Arc<AtomicBool>, led_animation: &Arc<Mutex<i32>>) {
    match event.event {
        EventType::ButtonChanged(Button::RightTrigger2, value, ..) => {
            let mut x = x.lock().expect("Failed to lock x");
            *x = (value * SPEED_SCALE) as i32;
        }
        EventType::ButtonChanged(Button::LeftTrigger2, value, ..) => {
            let mut x = x.lock().expect("Failed to lock x");
            *x = (-value * SPEED_SCALE) as i32;
        }
        EventType::AxisChanged(Axis::LeftStickX, value, ..) => {
            let mut y = y.lock().expect("Failed to lock y");
            *y = (value * SPEED_SCALE) as i32;
        }
        EventType::ButtonChanged(Button::DPadRight, value, ..) => {
            if value == 1.0 {
                let mut head_horizontal_rotation = head_horizontal_rotation.lock().expect("Failed to lock head_horizontal_rotation");
                if *head_horizontal_rotation < MAX_ROTATION {
                    *head_horizontal_rotation += ROTATION_STEP;
                }
            }
        }
        EventType::ButtonChanged(Button::DPadLeft, value, ..) => {
            if value == 1.0 {
                let mut head_horizontal_rotation = head_horizontal_rotation.lock().expect("Failed to lock head_horizontal_rotation");
                if *head_horizontal_rotation > MIN_ROTATION {
                    *head_horizontal_rotation -= ROTATION_STEP;
                }
            }
        }
        EventType::ButtonChanged(Button::DPadUp, value, ..) => {
            if value == 1.0 {
                let mut head_vertical_rotation = head_vertical_rotation.lock().expect("Failed to lock head_vertical_rotation");
                if *head_vertical_rotation < MAX_ROTATION {
                    *head_vertical_rotation += ROTATION_STEP;
                }
            }
        }
        EventType::ButtonChanged(Button::DPadDown, value, ..) => {
            if value == 1.0 {
                let mut head_vertical_rotation = head_vertical_rotation.lock().expect("Failed to lock head_vertical_rotation");
                if *head_vertical_rotation > MIN_ROTATION {
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
                let video_on = video_on.fetch_xor(true, Ordering::SeqCst);
            }
        }
        EventType::ButtonChanged(Button::South, value, ..) => {
            if value == 1.0 {
                let buzzer_on = buzzer_on.fetch_xor(true, Ordering::SeqCst);
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

fn emit_controller_state(app_handle: &tauri::AppHandle, x: &Arc<Mutex<i32>>, y: &Arc<Mutex<i32>>, face: &Arc<Mutex<i32>>, head_horizontal_rotation: &Arc<Mutex<i32>>, head_vertical_rotation: &Arc<Mutex<i32>>, video_on: &Arc<AtomicBool>, buzzer_on: &Arc<AtomicBool>, led_animation: &Arc<Mutex<i32>>) {
    let x = *x.lock().expect("Failed to lock x");
    let y = *y.lock().expect("Failed to lock y");
    let face = *face.lock().expect("Failed to lock face");
    let head_horizontal_rotation = *head_horizontal_rotation.lock().expect("Failed to lock head_horizontal_rotation");
    let head_vertical_rotation = *head_vertical_rotation.lock().expect("Failed to lock head_vertical_rotation");
    let head_rotation_array = vec![head_horizontal_rotation, head_vertical_rotation];
    let video_on = video_on.load(Ordering::SeqCst);
    let buzzer_on = buzzer_on.load(Ordering::SeqCst);
    let led_animation = *led_animation.lock().expect("Failed to lock led_animation");

    app_handle.emit_all(
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
    ).expect("Failed to emit controller state");
}
