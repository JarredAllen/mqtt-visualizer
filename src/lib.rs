use mqttrs::Packet;

pub fn format_packet(packet: Packet) -> String {
    match packet {
        Packet::Connect(packet) => format!("{:#?}\n", packet),
        Packet::Connack(packet) => format!("{:#?}\n", packet),
        Packet::Subscribe(packet) => format!("{:#?}\n", packet),
        Packet::Unsubscribe(packet) => format!("{:#?}\n", packet),
        Packet::Unsuback(packet) => format!("{:#?}\n", packet),
        Packet::Suback(packet) => format!("{:#?}\n", packet),
        Packet::Publish(packet) => format!("{:#?}\n", packet),
        Packet::Puback(packet) => format!("{:#?}\n", packet),
        Packet::Pubrel(packet) => format!("{:#?}\n", packet),
        Packet::Pubrec(packet) => format!("{:#?}\n", packet),
        Packet::Pubcomp(packet) => format!("{:#?}\n", packet),
        Packet::Disconnect => format!("{:#?}\n", Packet::Disconnect),
        Packet::Pingreq => format!("{:#?}\n", Packet::Disconnect),
        Packet::Pingresp => format!("{:#?}\n", Packet::Disconnect),
    }
}
