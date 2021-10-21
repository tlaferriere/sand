use crate::packet::Packet;
use sys_rust::ports;
use sys_rust::ports::{NBRead, Wait};

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
        let packet = match ports.pro_to_ic.wait().await {
            Ok(_) => match ports.pro_to_ic.read() {
                None => {
                    continue;
                }
                Some(packet) => packet.clone(),
            },
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

        ic_to_copro.write(packet).await;
        ic_to_copro_ready.write(true).await;

        let response = match copro_to_ic_ready.wait().await {
            Ok(_) => match copro_to_ic_ready.read() {
                None => {
                    eprintln!("Undefined value on signal copro_to_ic_ready");
                    continue;
                }
                Some(ready) => {
                    if *ready {
                        match copro_to_ic.read() {
                            None => {
                                continue;
                            }
                            Some(packet) => packet.clone(),
                        }
                    } else {
                        continue;
                    }
                }
            },
            Err(_) => {
                return;
            }
        };

        ports.ic_to_pro.write(response).await;
    }
}
