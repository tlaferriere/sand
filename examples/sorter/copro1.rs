use sys_rust::{In, Out};
use crate::packet::Packet;

pub(crate) struct Ports {
    pub(crate) ic_to_copro1_ready: In<bool>,
    pub(crate) ic_to_copro1: In<Packet>,
    pub(crate) copro1_to_ic_ready: Out<bool>,
    pub(crate) copro1_to_ic: Out<Packet>

}

pub(crate) fn process(ports: &mut Ports) {
    loop {
        let packet = match ports.ic_to_copro1_ready.wait() {
            Ok(_) =>
                match ports.ic_to_copro1_ready.read() {
                    None => {continue}
                    Some(ready) => if *ready {
                        match ports.ic_to_copro1.read() {
                            None => {continue}
                            Some(packet) => packet.clone()
                        }
                    } else {continue}
                }
            Err(_) => {return}
        };

        // TODO: Coprocess the payload here

        ports.copro1_to_ic.write(packet);
        ports.copro1_to_ic_ready.write(true);
    }
}
