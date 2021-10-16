use sys_rust::{In, Out};
use crate::packet::Packet;

pub(crate) struct Ports {
    pub(crate) ic_to_copro3_ready: In<bool>,
    pub(crate) ic_to_copro3: In<Packet>,
    pub(crate) copro3_to_ic_ready: Out<bool>,
    pub(crate) copro3_to_ic: Out<Packet>

}

pub(crate) fn process(ports: &mut Ports) {
    loop {
        let packet = match ports.ic_to_copro3_ready.wait() {
            Ok(_) =>
                match ports.ic_to_copro3_ready.read() {
                    None => {continue}
                    Some(ready) => if *ready {
                        match ports.ic_to_copro3.read() {
                            None => {continue}
                            Some(packet) => packet.clone()
                        }
                    } else {continue}
                }
            Err(_) => {return}
        };

        // TODO: Coprocess the payload here

        ports.copro3_to_ic.write(packet);
        ports.copro3_to_ic_ready.write(true);

    }
}
