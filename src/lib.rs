use mqttrs::Packet;

pub fn format_packet(packet: Packet) -> String {
    match packet {
        Packet::Connect(packet) => format!("{:#?}", packet),
        Packet::Connack(packet) => format!("{:#?}", packet),
        Packet::Subscribe(packet) => format!("{:#?}", packet),
        Packet::Unsubscribe(packet) => format!("{:#?}", packet),
        Packet::Unsuback(packet) => format!("{:#?}", packet),
        Packet::Suback(packet) => format!("{:#?}", packet),
        Packet::Publish(packet) => format!("{:#?}", packet),
        Packet::Puback(packet) => format!("{:#?}", packet),
        Packet::Pubrel(packet) => format!("{:#?}", packet),
        Packet::Pubrec(packet) => format!("{:#?}", packet),
        Packet::Pubcomp(packet) => format!("{:#?}", packet),
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
