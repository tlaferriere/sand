use crate::packet::Packet;
use futures::future::join_all;
use system_rust::ports;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::task;

mod copro1;
mod copro2;
mod copro3;
mod interconnect;
mod packet;
mod packet_gen;

#[tokio::main]
async fn main() {
    let (pro_to_ic_tx, pro_to_ic_rx): (Sender<Packet>, Receiver<Packet>) = channel(32);
    let (ic_to_pro_tx, ic_to_pro_rx): (Sender<Packet>, Receiver<Packet>) = channel(32);

    let (ic_to_copro1_tx, ic_to_copro1_rx): (Sender<Packet>, Receiver<Packet>) = channel(32);
    let (copro1_to_ic_tx, copro1_to_ic_rx): (Sender<Packet>, Receiver<Packet>) = channel(32);
    let (ic_to_copro1_ready_tx, ic_to_copro1_ready_rx): (Sender<bool>, Receiver<bool>) =
        channel(32);
    let (copro1_to_ic_ready_tx, copro1_to_ic_ready_rx): (Sender<bool>, Receiver<bool>) =
        channel(32);

    let (ic_to_copro2_tx, ic_to_copro2_rx): (Sender<Packet>, Receiver<Packet>) = channel(32);
    let (copro2_to_ic_tx, copro2_to_ic_rx): (Sender<Packet>, Receiver<Packet>) = channel(32);
    let (ic_to_copro2_ready_tx, ic_to_copro2_ready_rx): (Sender<bool>, Receiver<bool>) =
        channel(32);
    let (copro2_to_ic_ready_tx, copro2_to_ic_ready_rx): (Sender<bool>, Receiver<bool>) =
        channel(32);

    let (ic_to_copro3_tx, ic_to_copro3_rx): (Sender<Packet>, Receiver<Packet>) = channel(32);
    let (copro3_to_ic_tx, copro3_to_ic_rx): (Sender<Packet>, Receiver<Packet>) = channel(32);
    let (ic_to_copro3_ready_tx, ic_to_copro3_ready_rx): (Sender<bool>, Receiver<bool>) =
        channel(32);
    let (copro3_to_ic_ready_tx, copro3_to_ic_ready_rx): (Sender<bool>, Receiver<bool>) =
        channel(32);

    let children = vec![
        task::spawn(async move {
            let mut packet_gen_ports = packet_gen::Ports {
                pro_to_ic: ports::Out::new(pro_to_ic_tx),
                ic_to_pro: ports::In::new(ic_to_pro_rx),
            };
            packet_gen::process(&mut packet_gen_ports).await;
        }),
        task::spawn(async move {
            let mut interconnect_ports = interconnect::Ports {
                pro_to_ic: ports::In::new(pro_to_ic_rx),
                ic_to_pro: ports::Out::new(ic_to_pro_tx),

                ic_to_copro1_ready: ports::Out::new(ic_to_copro1_ready_tx),
                ic_to_copro1: ports::Out::new(ic_to_copro1_tx),
                copro1_to_ic_ready: ports::In::new(copro1_to_ic_ready_rx),
                copro1_to_ic: ports::In::new(copro1_to_ic_rx),

                ic_to_copro2_ready: ports::Out::new(ic_to_copro2_ready_tx),
                ic_to_copro2: ports::Out::new(ic_to_copro2_tx),
                copro2_to_ic_ready: ports::In::new(copro2_to_ic_ready_rx),
                copro2_to_ic: ports::In::new(copro2_to_ic_rx),

                ic_to_copro3_ready: ports::Out::new(ic_to_copro3_ready_tx),
                ic_to_copro3: ports::Out::new(ic_to_copro3_tx),
                copro3_to_ic_ready: ports::In::new(copro3_to_ic_ready_rx),
                copro3_to_ic: ports::In::new(copro3_to_ic_rx),
            };
            interconnect::process(&mut interconnect_ports).await;
        }),
        task::spawn(async move {
            let mut copro1_ports = copro1::Ports {
                ic_to_copro1_ready: ports::In::new(ic_to_copro1_ready_rx),
                ic_to_copro1: ports::In::new(ic_to_copro1_rx),
                copro1_to_ic_ready: ports::Out::new(copro1_to_ic_ready_tx),
                copro1_to_ic: ports::Out::new(copro1_to_ic_tx),
            };
            copro1::process(&mut copro1_ports).await;
        }),
        task::spawn(async move {
            let mut copro2 = copro2::Ports {
                ic_to_copro2_ready: ports::In::new(ic_to_copro2_ready_rx),
                ic_to_copro2: ports::In::new(ic_to_copro2_rx),
                copro2_to_ic_ready: ports::Out::new(copro2_to_ic_ready_tx),
                copro2_to_ic: ports::Out::new(copro2_to_ic_tx),
            };
            copro2::process(&mut copro2).await;
        }),
        task::spawn(async move {
            let mut copro3 = copro3::Ports {
                ic_to_copro3_ready: ports::In::new(ic_to_copro3_ready_rx),
                ic_to_copro3: ports::In::new(ic_to_copro3_rx),
                copro3_to_ic_ready: ports::Out::new(copro3_to_ic_ready_tx),
                copro3_to_ic: ports::Out::new(copro3_to_ic_tx),
            };
            copro3::process(&mut copro3).await;
        }),
    ];

    // Wait for the tasks to complete any remaining work
    join_all(children).await;
}
