use mqtt_topics::{temperature_data_topic, Esp};
use rand::Rng;
use rumqttc::{Client, MqttOptions, Packet, QoS};
use std::collections::HashMap;
use std::error::Error;

const UUID: &'static str = get_uuid::uuid();

#[derive(Debug)]
struct SensorData {
    temperature: Option<f32>,
    humidity: Option<f32>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let client_id = UUID;
    let mut sensor_data: HashMap<String, SensorData> = HashMap::new();
    sensor_data.insert(
        String::from("ESP4"),
        SensorData {
            temperature: None,
            humidity: None,
        },
    );
    sensor_data.insert(
        String::from("ESP3"),
        SensorData {
            temperature: None,
            humidity: None,
        },
    );

    let mqtt_host = "test.mosquitto.org";
    dbg!(UUID);

    let mqttoptions = MqttOptions::new(client_id, mqtt_host, 1883);

    let (mut client, mut connection) = Client::new(mqttoptions, 100);
    client.subscribe("sensor_data/temperature", QoS::AtMostOnce)?;
    client.subscribe("sensor_data/humidity", QoS::AtMostOnce)?;

    // Iterate to poll the eventloop for connection progress
    for (_, notification) in connection.iter().enumerate() {
        // if you want to see *everything*, uncomment:

        for (key, value) in &sensor_data {
            println!("{}: {:#?}", key, value);
        }

        if let Ok(rumqttc::Event::Incoming(Packet::Publish(publish_data))) = notification {
            if publish_data.topic == "sensor_data/temperature" {
                let (id, temp_val) = deconstruct_message(&publish_data.payload).unwrap();
                //println!("{} : {:?} C", id, temp_val);

                if let Some(sample) = sensor_data.get_mut("ESP3") {
                    sample.temperature = Some(add_noise(temp_val, 5.0));
                }

                if let Some(sample) = sensor_data.get_mut("ESP4") {
                    sample.temperature = Some(add_noise(temp_val, 3.0));
                }

                if let Some(sample) = sensor_data.get_mut(id) {
                    sample.temperature = Some(temp_val);
                } else {
                    sensor_data.insert(
                        id.to_string(),
                        SensorData {
                            temperature: Some(temp_val),
                            humidity: None,
                        },
                    );
                }
            }
            if publish_data.topic == "sensor_data/humidity" {
                let (id, hum_val) = deconstruct_message(&publish_data.payload).unwrap();
                //println!("{} : {:?} %", id, hum_val);

                if let Some(sample) = sensor_data.get_mut("ESP3") {
                    sample.humidity = Some(add_noise(hum_val, 4.0));
                }

                if let Some(sample) = sensor_data.get_mut("ESP4") {
                    sample.humidity = Some(add_noise(hum_val, 5.0));
                }

                if let Some(sample) = sensor_data.get_mut(id) {
                    sample.humidity = Some(hum_val);
                } else {
                    sensor_data.insert(
                        id.to_string(),
                        SensorData {
                            temperature: None,
                            humidity: Some(hum_val),
                        },
                    );
                }
            }
        }
    }
    Ok(())
}

fn deconstruct_message(msg: &[u8]) -> Option<(&str, f32)> {
    let id = msg[0..36].try_into();

    let data: Result<[u8; 4], std::array::TryFromSliceError> = msg[36..40].try_into();

    if let (Ok(data), Ok(id)) = (data, id) {
        let temp_val = f32::from_le_bytes(data);
        if let Ok(id) = core::str::from_utf8(id) {
            return Some((id, temp_val));
        }
    }
    None
}

fn add_noise(original: f32, range: f32) -> f32 {
    let mut rng = rand::thread_rng();
    let noise = rng.gen_range(-range..=range);
    original + noise
}
