use crate::packet::Packet;
use system_rust::{ports, Read, Write};

pub(crate) struct Ports {
    pub(crate) ic_to_copro2_ready: ports::In<bool>,
    pub(crate) ic_to_copro2: ports::In<Packet>,
    pub(crate) copro2_to_ic_ready: ports::Out<bool>,
    pub(crate) copro2_to_ic: ports::Out<Packet>,
}

pub(crate) async fn process(ports: &mut Ports) {
    loop {
        let packet = match ports.ic_to_copro2_ready.b_read().await {
            Err(_) => return,
            Ok(ready) => {
                if ready {
                    match ports.ic_to_copro2.nb_read() {
                        Err(_) => continue,
                        Ok(packet) => packet.clone(),
                    }
                } else {
                    continue;
                }
            }
        };

        // TODO: Coprocess the payload here

        ports.copro2_to_ic.nb_write(packet);
        ports.copro2_to_ic_ready.nb_write(true);
    }
}
