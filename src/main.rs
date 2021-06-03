use std::io::{prelude::*, stdin};

fn main() {
    for line in stdin().lock().lines() {
        let data = hex::decode(line.unwrap()).expect("Error decoding hex");
        let packet = mqttrs::decode_slice(&data).expect("Error decoding slice").expect("No packet found");
        println!("{:#?}", packet);
    }
}
