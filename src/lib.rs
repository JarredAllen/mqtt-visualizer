use mqttrs::Packet;
use std::convert::TryFrom;
use std::string::FromUtf8Error;

#[derive(Debug)]
struct Publish {
    dup: bool,
    qospid: mqttrs::QosPid,
    retain: bool,
    topic_name: String,
    payload: String
}
impl<'a> TryFrom<&'a mqttrs::Publish<'a>> for Publish {
    type Error = FromUtf8Error;

    fn try_from(publish: &'a mqttrs::Publish<'a>) -> Result<Self, FromUtf8Error> {
        Ok(Self {
            dup: publish.dup,
            qospid: publish.qospid,
            retain: publish.retain,
            topic_name: publish.topic_name.to_owned(),
            payload: String::from_utf8(publish.payload.into())?
        })
    }
}

pub fn format_packet(packet: Packet) -> String {
    match packet {
        Packet::Connect(packet) => format!("{:#?}", packet),
        Packet::Connack(packet) => format!("{:#?}", packet),
        Packet::Subscribe(packet) => format!("{:#?}", packet),
        Packet::Unsubscribe(packet) => format!("{:#?}", packet),
        Packet::Suback(packet) => format!("{:#?}", packet),
        Packet::Publish(packet) => {
            if let Ok(packet) = Publish::try_from(&packet) {
                format!("{:#?}", packet)
            } else {
                format!("{:#?}", packet)
            }
        },
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
