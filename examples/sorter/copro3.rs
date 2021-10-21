use crate::packet::Packet;
use sys_rust::ports;
use sys_rust::ports::{NBRead, Wait};

pub(crate) struct Ports {
    pub(crate) ic_to_copro3_ready: ports::In<bool>,
    pub(crate) ic_to_copro3: ports::In<Packet>,
    pub(crate) copro3_to_ic_ready: ports::Out<bool>,
    pub(crate) copro3_to_ic: ports::Out<Packet>,
}

pub(crate) async fn process(ports: &mut Ports) {
    loop {
        let packet = match ports.ic_to_copro3_ready.wait().await {
            Ok(_) => match ports.ic_to_copro3_ready.read() {
                None => continue,
                Some(ready) => {
                    if *ready {
                        match ports.ic_to_copro3.read() {
                            None => continue,
                            Some(packet) => packet.clone(),
                        }
                    } else {
                        continue;
                    }
                }
            },
            Err(_) => return,
        };

        // TODO: Coprocess the payload here

        ports.copro3_to_ic.write(packet).await;
        ports.copro3_to_ic_ready.write(true).await;
    }
}
