extern crate sys_rust;

use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use sys_rust::{In, Out};
use crate::packet::Packet;

mod packet_gen;
mod interconnect;
mod copro3;
mod copro2;
mod copro1;
mod packet;

// #[tokio::main]
fn main() {
    let (pro_to_ic_tx, pro_to_ic_rx): (Sender<Packet>, Receiver<Packet>) = channel();
    let (ic_to_pro_tx, ic_to_pro_rx): (Sender<Packet>, Receiver<Packet>) = channel();

    let (ic_to_copro1_tx, ic_to_copro1_rx): (Sender<Packet>, Receiver<Packet>) = channel();
    let (copro1_to_ic_tx, copro1_to_ic_rx): (Sender<Packet>, Receiver<Packet>) = channel();
    let (ic_to_copro1_ready_tx, ic_to_copro1_ready_rx): (Sender<bool>, Receiver<bool>) = channel();
    let (copro1_to_ic_ready_tx, copro1_to_ic_ready_rx): (Sender<bool>, Receiver<bool>) = channel();

    let (ic_to_copro2_tx, ic_to_copro2_rx): (Sender<Packet>, Receiver<Packet>) = channel();
    let (copro2_to_ic_tx, copro2_to_ic_rx): (Sender<Packet>, Receiver<Packet>) = channel();
    let (ic_to_copro2_ready_tx, ic_to_copro2_ready_rx): (Sender<bool>, Receiver<bool>) = channel();
    let (copro2_to_ic_ready_tx, copro2_to_ic_ready_rx): (Sender<bool>, Receiver<bool>) = channel();

    let (ic_to_copro3_tx, ic_to_copro3_rx): (Sender<Packet>, Receiver<Packet>) = channel();
    let (copro3_to_ic_tx, copro3_to_ic_rx): (Sender<Packet>, Receiver<Packet>) = channel();
    let (ic_to_copro3_ready_tx, ic_to_copro3_ready_rx): (Sender<bool>, Receiver<bool>) = channel();
    let (copro3_to_ic_ready_tx, copro3_to_ic_ready_rx): (Sender<bool>, Receiver<bool>) = channel();

    let mut children = vec!(
        thread::spawn(move || {
            let mut packet_gen_ports = packet_gen::Ports {
                pro_to_ic: Out::new(pro_to_ic_tx),
                ic_to_pro: In::new(ic_to_pro_rx)
            };
            packet_gen::process(&mut packet_gen_ports);
        }),
        thread::spawn(move || {
            let mut interconnect_ports = interconnect::Ports {
                pro_to_ic:          In::new(pro_to_ic_rx),
                ic_to_pro:          Out::new(ic_to_pro_tx),

                ic_to_copro1_ready: Out::new(ic_to_copro1_ready_tx),
                ic_to_copro1:       Out::new(ic_to_copro1_tx),
                copro1_to_ic_ready: In::new(copro1_to_ic_ready_rx),
                copro1_to_ic:       In::new(copro1_to_ic_rx),

                ic_to_copro2_ready: Out::new(ic_to_copro2_ready_tx),
                ic_to_copro2:       Out::new(ic_to_copro2_tx),
                copro2_to_ic_ready: In::new(copro2_to_ic_ready_rx),
                copro2_to_ic:       In::new(copro2_to_ic_rx),

                ic_to_copro3_ready: Out::new(ic_to_copro3_ready_tx),
                ic_to_copro3:       Out::new(ic_to_copro3_tx),
                copro3_to_ic_ready: In::new(copro3_to_ic_ready_rx),
                copro3_to_ic:       In::new(copro3_to_ic_rx)
            };
            interconnect::process(&mut interconnect_ports);
        }),
        thread::spawn(move || {
            let mut copro1_ports = copro1::Ports {
                ic_to_copro1_ready: In::new(ic_to_copro1_ready_rx),
                ic_to_copro1:       In::new(ic_to_copro1_rx),
                copro1_to_ic_ready: Out::new(copro1_to_ic_ready_tx),
                copro1_to_ic:       Out::new(copro1_to_ic_tx)
            };
            copro1::process(&mut copro1_ports);
        }),
        thread::spawn(move || {
            let mut copro2 = copro2::Ports {
                ic_to_copro2_ready: In::new(ic_to_copro2_ready_rx),
                ic_to_copro2:       In::new(ic_to_copro2_rx),
                copro2_to_ic_ready: Out::new(copro2_to_ic_ready_tx),
                copro2_to_ic:       Out::new(copro2_to_ic_tx)
            };
            copro2::process(&mut copro2);
        }),
        thread::spawn(move || {
            let mut copro3 = copro3::Ports {
                ic_to_copro3_ready: In::new(ic_to_copro3_ready_rx),
                ic_to_copro3:       In::new(ic_to_copro3_rx),
                copro3_to_ic_ready: Out::new(copro3_to_ic_ready_tx),
                copro3_to_ic:       Out::new(copro3_to_ic_tx)
            };
            copro3::process(&mut copro3);
        }),
    );

    // Wait for the threads to complete any remaining work
    for child in children {
        child.join().expect("oops! the child thread panicked");
    }
}
