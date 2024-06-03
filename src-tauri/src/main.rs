// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use gilrs::{Button, EventType, Gilrs};
use serde_json::json;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tauri::Manager;

fn main() {
    let mut gilrs = Gilrs::new().unwrap();
    let x = Arc::new(Mutex::new(0));
    let y = Arc::new(Mutex::new(0));
    let face = Arc::new(Mutex::new(0));
    let head_horizontal_rotation = Arc::new(Mutex::new(0));
    let head_vertical_rotation = Arc::new(Mutex::new(0));
    // video on is either 0 or 1
    let video_on = Arc::new(Mutex::new(0));

    let x_clone = Arc::clone(&x);
    let y_clone = Arc::clone(&y);
    let face_clone = Arc::clone(&face);
    let head_horizontal_rotation_clone = Arc::clone(&head_horizontal_rotation);
    let head_vertical_rotation_clone = Arc::clone(&head_vertical_rotation);
    let video_on_clone = Arc::clone(&video_on);

    thread::spawn(move || {
        loop {
            while let Some(event) = gilrs.next_event() {
                match event.event {
                    // We use the triggers to control the speed of the robot Right is forward and left is backward
                    EventType::ButtonChanged(Button::RightTrigger2, value, ..) => {
                        let mut x = x_clone.lock().unwrap();
                        *x = (value * 100.0) as i32; // Scale to 0-100 range
                    }
                    EventType::ButtonChanged(Button::LeftTrigger2, value, ..) => {
                        let mut x = x_clone.lock().unwrap();
                        *x = (-value * 100.0) as i32; // Scale to 0-100 range and negate
                    }
                    // We use LeftStick to control the movement of the robot
                    EventType::AxisChanged(gilrs::Axis::LeftStickX, value, ..) => {
                        let mut y = y_clone.lock().unwrap();
                        *y = (value * 100.0) as i32;
                    }
                    // We use RightStick to control the head rotation up is up down is down left is left and right is right
                    EventType::AxisChanged(gilrs::Axis::RightStickX, value, ..) => {
                        let mut head_horizontal_rotation =
                            head_horizontal_rotation_clone.lock().unwrap();
                        *head_horizontal_rotation = (value.max(0.0).min(1.0) * 180.0) as i32;
                    }
                    EventType::AxisChanged(gilrs::Axis::RightStickY, value, ..) => {
                        let mut head_vertical_rotation =
                            head_vertical_rotation_clone.lock().unwrap();
                        *head_vertical_rotation = (value.max(0.0).min(1.0) * 180.0) as i32;
                    }
                    // To cycle through the faces we use the DPad Down and Up buttons
                    EventType::ButtonChanged(Button::DPadDown, value, ..) => {
                        if value == 1.0 {
                            let mut face = face_clone.lock().unwrap();
                            *face = (*face + 1) % 8;
                        }
                    }
                    EventType::ButtonChanged(Button::DPadUp, value, ..) => {
                        if value == 1.0 {
                            let mut face = face_clone.lock().unwrap();
                            *face = (*face + 7) % 8;
                        }
                    }
                    // We use the south button to toggle the video on and off
                    EventType::ButtonChanged(Button::South, value, ..) => {
                        if value == 1.0 {
                            let mut video_on = video_on_clone.lock().unwrap();
                            *video_on = 1 - *video_on;
                        }
                    }
                    _ => {}
                }
            }
            thread::sleep(Duration::from_millis(10));
        }
    });

    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle();
            thread::spawn(move || loop {
                let x = *x.lock().unwrap();
                let y = *y.lock().unwrap();
                let face = *face.lock().unwrap();
                let head_horizontal_rotation = *head_horizontal_rotation.lock().unwrap();
                let head_vertical_rotation = *head_vertical_rotation.lock().unwrap();
                let head_rotation_array = vec![head_horizontal_rotation, head_vertical_rotation];
                let video_on = *video_on.lock().unwrap();
                app_handle
                    .emit_all(
                        "controller",
                        json!({ "x": x, "y": y, "face": face, "headRotation": head_rotation_array, "videoOn": video_on}),
                    )
                    .unwrap();
                thread::sleep(Duration::from_millis(100));
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
