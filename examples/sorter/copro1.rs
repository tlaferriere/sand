use crate::packet::Packet;
use system_rust::{ports, Read, Write};

ports! {
    ic_to_copro1_ready <- bool,
    ic_to_copro1 <- Packet,
    copro1_to_ic_ready -> bool,
    copro1_to_ic -> Packet,
}

pub(crate) async fn process(ports: &mut Ports) {
    loop {
        let packet = match ports.ic_to_copro1_ready.b_read().await {
            Ok(ready) => {
                if ready {
                    match ports.ic_to_copro1.nb_read() {
                        Err(_) => continue,
                        Ok(packet) => packet.clone(),
                    }
                } else {
                    continue;
                }
            }
            Err(_) => return,
        };

        // TODO: Coprocess the payload here

        ports.copro1_to_ic.nb_write(packet);
        ports.copro1_to_ic_ready.nb_write(true);
    }
}
