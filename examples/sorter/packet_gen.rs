use std::sync::mpsc::{Receiver, RecvError, Sender};
use crate::packet::Packet;
use rand::Rng;
use sys_rust::{In, Out};

pub(crate) struct Ports {
    pub(crate) pro_to_ic: Out<Packet>,
    pub(crate) ic_to_pro: In<Packet>
}

pub(crate) fn process(ports: &mut Ports) {
    let mut rng = rand::thread_rng();
    for address in 0..4 {
        let payload: Vec<u32> = (0..10).map(|_| rng.gen_range(0..1000)).collect();
        let payload_size = payload.len() as u32;
        let packet = Packet {
            id: address,
            address,
            payload,
            payload_size
        };
        ports.pro_to_ic.write(packet.clone());

        match ports.ic_to_pro.wait() {
            Ok(_) => {
                match ports.ic_to_pro.read() {
                    None => {
                        eprintln!("Error: Undefined value after wait");
                    },
                    Some(response) => {
                        let mut check = true;
                        for i in 0..response.payload.len() {
                            if response.payload[i] != packet.payload[i] {
                                check = false;
                                break;
                            }
                        }
                        if check {
                            println!("Yay!");
                        } else {
                            println!("Nay!");
                        }
                    }
                };
            }
            Err(err) => {
                eprintln!("Receive Error: {}", err);
            }
        };
    }
}
