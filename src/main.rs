use std::io::{prelude::*, stdin};

use mqtt_visualizer::format_packet;

fn main() {
    for line in stdin().lock().lines() {
        let line = line.unwrap();
        let data = hex::decode(line.clone()).expect("Error decoding hex");
        let packet = mqttrs::decode_slice(&data)
            .expect("Error decoding slice")
            .expect("No packet found");
        println!("Data: {}", line);
        println!("{}\n", format_packet(packet));
    }
}
