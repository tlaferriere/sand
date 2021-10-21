use crate::packet::Packet;
use rand::Rng;
use system_rust::ports;
use system_rust::ports::{NBRead, Wait};

pub(crate) struct Ports {
    pub(crate) pro_to_ic: ports::Out<Packet>,
    pub(crate) ic_to_pro: ports::In<Packet>,
}

pub(crate) async fn process(ports: &mut Ports) {
    for address in 0..4 {
        let payload: Vec<u32>;
        {
            let mut rng = rand::thread_rng();
            payload = (0..10).map(|_| rng.gen_range(0..1000)).collect();
        }
        let payload_size = payload.len() as u32;
        let packet = Packet {
            id: address,
            address,
            payload,
            payload_size,
        };
        ports.pro_to_ic.write(packet.clone()).await;

        match ports.ic_to_pro.wait().await {
            Ok(_) => {
                match ports.ic_to_pro.read() {
                    None => {
                        eprintln!("Error: Undefined value after wait");
                    }
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
            Err(_) => {
                eprintln!("Wait error");
            }
        };
    }
}
