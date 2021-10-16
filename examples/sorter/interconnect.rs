use std::sync::mpsc::RecvError;
use sys_rust::{In, Out};
use crate::packet::Packet;

pub(crate) struct Ports {
    pub(crate) pro_to_ic: In<Packet>,
    pub(crate) ic_to_pro: Out<Packet>,

    pub(crate) ic_to_copro1_ready: Out<bool>,
    pub(crate) ic_to_copro1: Out<Packet>,
    pub(crate) copro1_to_ic_ready: In<bool>,
    pub(crate) copro1_to_ic: In<Packet>,

    pub(crate) ic_to_copro2_ready: Out<bool>,
    pub(crate) ic_to_copro2: Out<Packet>,
    pub(crate) copro2_to_ic_ready: In<bool>,
    pub(crate) copro2_to_ic: In<Packet>,

    pub(crate) ic_to_copro3_ready: Out<bool>,
    pub(crate) ic_to_copro3: Out<Packet>,
    pub(crate) copro3_to_ic_ready: In<bool>,
    pub(crate) copro3_to_ic: In<Packet>,
}

pub(crate) fn process(ports: &mut Ports) {
    loop {
        let packet = match ports.pro_to_ic.wait() {
            Ok(_) =>
                match ports.pro_to_ic.read() {
                    None => { continue; }
                    Some(packet) => packet.clone()
                }
            Err(_) => { return; }
        };

        // Dispatch the packet to the right coprocessor by address.
        let (ic_to_copro, ic_to_copro_ready, copro_to_ic_ready, copro_to_ic):
            (&mut Out<Packet>, &mut Out<bool>, &mut In<bool>, &mut In<Packet>) =
            match packet.address {
            0 => (&mut ports.ic_to_copro1,
                  &mut ports.ic_to_copro1_ready,
                  &mut ports.copro1_to_ic_ready,
                  &mut ports.copro1_to_ic),
            1 => (&mut ports.ic_to_copro2,
                  &mut ports.ic_to_copro2_ready,
                  &mut ports.copro2_to_ic_ready,
                  &mut ports.copro2_to_ic),
            2 => (&mut ports.ic_to_copro3,
                  &mut ports.ic_to_copro3_ready,
                  &mut ports.copro3_to_ic_ready,
                  &mut ports.copro3_to_ic),
            address => {
                eprintln!("Bad packet address: {}", address);
                return;
            }
        };

        ic_to_copro.write(packet);
        ic_to_copro_ready.write(true);

        let response = match copro_to_ic_ready.wait() {
            Ok(_) =>
                match copro_to_ic_ready.read() {
                    None => {
                        eprintln!("Undefined value on signal copro_to_ic_ready");
                        continue;
                    }
                    Some(ready) => if *ready {
                        match copro_to_ic.read() {
                            None => { continue; }
                            Some(packet) => packet.clone()
                        }
                    } else { continue; }
                }
            Err(err) => { return; }
        };


        ports.ic_to_pro.write(response);
    }
}