use crate::packet::Packet;
use system_rust::{ports, Read, Write};

pub(crate) struct Ports {
    pub(crate) pro_to_ic: ports::In<Packet>,
    pub(crate) ic_to_pro: ports::Out<Packet>,

    pub(crate) ic_to_copro1_ready: ports::Out<bool>,
    pub(crate) ic_to_copro1: ports::Out<Packet>,
    pub(crate) copro1_to_ic_ready: ports::In<bool>,
    pub(crate) copro1_to_ic: ports::In<Packet>,

    pub(crate) ic_to_copro2_ready: ports::Out<bool>,
    pub(crate) ic_to_copro2: ports::Out<Packet>,
    pub(crate) copro2_to_ic_ready: ports::In<bool>,
    pub(crate) copro2_to_ic: ports::In<Packet>,

    pub(crate) ic_to_copro3_ready: ports::Out<bool>,
    pub(crate) ic_to_copro3: ports::Out<Packet>,
    pub(crate) copro3_to_ic_ready: ports::In<bool>,
    pub(crate) copro3_to_ic: ports::In<Packet>,
}

pub(crate) async fn process(ports: &mut Ports) {
    loop {
        let packet = match ports.pro_to_ic.b_read().await {
            Ok(packet) => packet.clone(),
            Err(_) => {
                return;
            }
        };

        // Dispatch the packet to the right coprocessor by address.
        let (ic_to_copro, ic_to_copro_ready, copro_to_ic_ready, copro_to_ic): (
            &mut ports::Out<Packet>,
            &mut ports::Out<bool>,
            &mut ports::In<bool>,
            &mut ports::In<Packet>,
        ) = match packet.address {
            0 => (
                &mut ports.ic_to_copro1,
                &mut ports.ic_to_copro1_ready,
                &mut ports.copro1_to_ic_ready,
                &mut ports.copro1_to_ic,
            ),
            1 => (
                &mut ports.ic_to_copro2,
                &mut ports.ic_to_copro2_ready,
                &mut ports.copro2_to_ic_ready,
                &mut ports.copro2_to_ic,
            ),
            2 => (
                &mut ports.ic_to_copro3,
                &mut ports.ic_to_copro3_ready,
                &mut ports.copro3_to_ic_ready,
                &mut ports.copro3_to_ic,
            ),
            address => {
                eprintln!("Bad packet address: {}", address);
                return;
            }
        };

        ic_to_copro.nb_write(packet);
        ic_to_copro_ready.nb_write(true);

        let response = match copro_to_ic_ready.b_read().await {
            Ok(ready) => {
                if ready {
                    match copro_to_ic.nb_read() {
                        Err(_) => {
                            continue;
                        }
                        Ok(packet) => packet.clone(),
                    }
                } else {
                    continue;
                }
            }
            Err(_) => {
                return;
            }
        };

        ports.ic_to_pro.nb_write(response);
    }
}
