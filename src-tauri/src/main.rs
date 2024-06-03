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
    // We use the D-pad to change the faces up d-pad adds 1, down subtracts 1
    // if we go above 7 we go back to 0, if we go below 0 we go to 7
    let face = Arc::new(Mutex::new(0));

    let x_clone = Arc::clone(&x);
    let y_clone = Arc::clone(&y);
    let face_clone = Arc::clone(&face);

    thread::spawn(move || {
        loop {
            while let Some(event) = gilrs.next_event() {
                match event.event {
                    EventType::ButtonChanged(Button::RightTrigger2, value, ..) => {
                        let mut x = x_clone.lock().unwrap();
                        *x = (value * 100.0) as i32; // Scale to 0-100 range
                    }
                    EventType::ButtonChanged(Button::LeftTrigger2, value, ..) => {
                        let mut x = x_clone.lock().unwrap();
                        *x = (-value * 100.0) as i32; // Scale to 0-100 range and negate
                    }
                    EventType::AxisChanged(gilrs::Axis::LeftStickX, value, ..) => {
                        let mut y = y_clone.lock().unwrap();
                        *y = (value * 100.0) as i32;
                    }
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
                app_handle
                    .emit_all("controller", json!({ "x": x, "y": y, "face": face}))
                    .unwrap();
                thread::sleep(Duration::from_millis(100));
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
