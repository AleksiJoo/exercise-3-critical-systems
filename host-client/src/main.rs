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
    client.subscribe(
        temperature_data_topic(UUID, Esp::EspTarget1).as_str(),
        QoS::AtMostOnce,
    )?;

    // Iterate to poll the eventloop for connection progress
    for (_, notification) in connection.iter().enumerate() {
        // if you want to see *everything*, uncomment:
        // println!("Notification = {:#?}", notification);
        if let Ok(rumqttc::Event::Incoming(Packet::Publish(publish_data))) = notification {
            if publish_data.topic == temperature_data_topic(UUID, Esp::EspTarget1).as_str() {
                let data: &[u8] = &publish_data.payload;
                println!("{:?}", data);
                let data: Result<[u8; 4], _> = data.try_into();

                if let Ok(data) = data {
                    println!("{:?}", data)
                }
            }
        }
    }
    Ok(())
}
