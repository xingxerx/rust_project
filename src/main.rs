use reqwest::Client;
use serde::{Serialize, Deserialize};
use chrono::Utc;
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
struct SensorData {
    temperature: f32,
    humidity: f32,
    timestamp: i64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Replace with your actual Firebase Realtime Database URL.
    // For example, "https://your-project-id.firebaseio.com/sensor_data.json"
    let firebase_url = "https://your-database.firebaseio.com/sensor_data.json";

    // Create some simulated sensor data.
    let data = SensorData {
        temperature: 23.5,
        humidity: 60.0,
        timestamp: Utc::now().timestamp(),
    };

    // Create a reqwest client and send a POST request with your data.
    let client = Client::new();
    let response = client.post(firebase_url)
        .json(&data)
        .send()
        .await?;

    // Print the response from Firebase.
    let response_text = response.text().await?;
    println!("Firebase Response: {}", response_text);

    Ok(())
}