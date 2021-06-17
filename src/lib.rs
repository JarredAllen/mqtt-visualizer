use mqttrs::Packet;

pub fn format_packet(packet: Packet) -> String {
    match packet {
        Packet::Connect(packet) => format!("{:#?}", packet),
        Packet::Connack(packet) => format!("{:#?}", packet),
        Packet::Subscribe(packet) => format!("{:#?}", packet),
        Packet::Unsubscribe(packet) => format!("{:#?}", packet),
        Packet::Suback(packet) => format!("{:#?}", packet),
        Packet::Publish(packet) => format!("{:#?}\nPayload: {:?}", packet, String::from_utf8_lossy(packet.payload)),
        packet
        @
        (Packet::Puback(_)
        | Packet::Pubrel(_)
        | Packet::Pubrec(_)
        | Packet::Pubcomp(_)
        | Packet::Unsuback(_)) => format!("{:#?}", packet),
        Packet::Disconnect => format!("{:#?}", Packet::Disconnect),
        Packet::Pingreq => format!("{:#?}", Packet::Disconnect),
        Packet::Pingresp => format!("{:#?}", Packet::Disconnect),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_disconnect() {
        assert_eq!("Disconnect", format_packet(Packet::Disconnect));
    }
}
