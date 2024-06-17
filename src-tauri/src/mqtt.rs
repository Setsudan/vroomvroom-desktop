use rumqttc::{Client, MqttOptions, QoS};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tauri::Manager;
use tokio::task;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SensorData {
    pub battery_voltage: f32,
    pub photosensitive_value: u32,
    pub tracking_module_values: u32,
    pub ultrasonic_distance: f32,
    pub motor_speeds: [u32; 4],
    pub buzzer_status: String,
    pub buzzer_frequency: u32,
}

pub async fn mqtt_subscribe(handle: tauri::AppHandle) {
    let mut mqttoptions = MqttOptions::new("tauri_app", "192.168.64.40", 1883);

    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (client, mut connection) = Client::new(mqttoptions, 10);
    client.subscribe("car/sensors", QoS::AtMostOnce).unwrap();

    task::spawn(async move {
        for notification in connection.iter() {
            if let Ok(notification) = notification {
                if let rumqttc::Event::Incoming(rumqttc::Packet::Publish(publish)) = notification {
                    let payload = String::from_utf8(publish.payload.to_vec()).unwrap();
                    if let Ok(sensor_data) = serde_json::from_str::<SensorData>(&payload) {
                        handle.emit_all("sensor-data", sensor_data).unwrap();
                    }
                }
            }
        }
    });
}
