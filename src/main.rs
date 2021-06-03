use std::io::{prelude::*, stdin};
use mqttrs::Packet;

fn main() {
    for line in stdin().lock().lines() {
        let line = line.unwrap();
        let data = hex::decode(line.clone()).expect("Error decoding hex");
        let packet = mqttrs::decode_slice(&data).expect("Error decoding slice").expect("No packet found");
        println!("Data: {}", line);
        print_packet(packet);
    }
}

fn print_packet(packet: Packet) {
    match packet {
        Packet::Connect(packet) => println!("{:#?}\n", packet),
        Packet::Connack(packet) => println!("{:#?}\n", packet),
        Packet::Subscribe(packet) => println!("{:#?}\n", packet),
        Packet::Unsubscribe(packet) => println!("{:#?}\n", packet),
        Packet::Unsuback(packet) => println!("{:#?}\n", packet),
        Packet::Suback(packet) => println!("{:#?}\n", packet),
        Packet::Publish(packet) => println!("{:#?}\n", packet),
        Packet::Puback(packet) => println!("{:#?}\n", packet),
        Packet::Pubrel(packet) => println!("{:#?}\n", packet),
        Packet::Pubrec(packet) => println!("{:#?}\n", packet),
        Packet::Pubcomp(packet) => println!("{:#?}\n", packet),
        Packet::Disconnect => println!("{:#?}\n", Packet::Disconnect),
        Packet::Pingreq => println!("{:#?}\n", Packet::Disconnect),
        Packet::Pingresp => println!("{:#?}\n", Packet::Disconnect),
    }
}
