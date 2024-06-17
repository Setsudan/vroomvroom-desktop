#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use std::time::Duration;

mod controller;
mod mqtt;

#[tokio::main]
async fn main() {
    let x = Arc::new(Mutex::new(0));
    let y = Arc::new(Mutex::new(0));
    let face = Arc::new(Mutex::new(0));
    let head_horizontal_rotation = Arc::new(Mutex::new(90));
    let head_vertical_rotation = Arc::new(Mutex::new(90));
    let video_on = Arc::new(AtomicBool::new(false));
    let buzzer_on = Arc::new(AtomicBool::new(false));
    let led_animation = Arc::new(Mutex::new(0));

    controller::handle_controller_events(
        Arc::clone(&x),
        Arc::clone(&y),
        Arc::clone(&face),
        Arc::clone(&head_horizontal_rotation),
        Arc::clone(&head_vertical_rotation),
        Arc::clone(&video_on),
        Arc::clone(&buzzer_on),
        Arc::clone(&led_animation),
    );

    tauri::Builder::default()
        .setup(move |app| {
            let app_handle = app.handle();
            let x = Arc::clone(&x);
            let y = Arc::clone(&y);
            let face = Arc::clone(&face);
            let head_horizontal_rotation = Arc::clone(&head_horizontal_rotation);
            let head_vertical_rotation = Arc::clone(&head_vertical_rotation);
            let video_on = Arc::clone(&video_on);
            let buzzer_on = Arc::clone(&buzzer_on);
            let led_animation = Arc::clone(&led_animation);

            tokio::spawn(async move {
                loop {
                    controller::emit_controller_state(
                        &app_handle,
                        &x,
                        &y,
                        &face,
                        &head_horizontal_rotation,
                        &head_vertical_rotation,
                        &video_on,
                        &buzzer_on,
                        &led_animation,
                    );
                    tokio::time::sleep(Duration::from_millis(100)).await;
                }
            });

            let handle = app.handle().clone();
            tokio::spawn(async move {
                mqtt::mqtt_subscribe(handle).await;
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
