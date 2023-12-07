use mqtt_topics::{temperature_data_topic, Esp};
use rumqttc::{Client, MqttOptions, Packet, QoS};
use std::error::Error;

const UUID: &'static str = get_uuid::uuid();

fn main() -> Result<(), Box<dyn Error>> {
    let client_id = UUID;
    let mqtt_host = "test.mosquitto.org";
    dbg!(UUID);

    let mqttoptions = MqttOptions::new(client_id, mqtt_host, 1883);

    let (mut client, mut connection) = Client::new(mqttoptions, 100);
    client.subscribe("sensor_data/temperature", QoS::AtMostOnce)?;
    client.subscribe("sensor_data/humidity", QoS::AtMostOnce)?;

    // Iterate to poll the eventloop for connection progress
    for (_, notification) in connection.iter().enumerate() {
        // if you want to see *everything*, uncomment:
        if let Ok(rumqttc::Event::Incoming(Packet::Publish(publish_data))) = notification {
            if publish_data.topic == "sensor_data/temperature" {
                let (id, temp_val) = deconstruct_message(&publish_data.payload).unwrap();
                println!("{} : {:?} C", id, temp_val);
            }
            if publish_data.topic == "sensor_data/humidity" {
                let (id, hum_val) = deconstruct_message(&publish_data.payload).unwrap();
                println!("{} : {:?} %", id, hum_val);
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
