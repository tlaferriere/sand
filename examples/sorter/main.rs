use crate::packet::Packet;
use futures::future::join_all;
use system_rust::ports;
use system_rust::signals::signal::Signal;
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
    let pro_to_ic = Signal::new();
    let ic_to_pro = Signal::new();

    let ic_to_copro1 = Signal::new();
    let copro1_to_ic = Signal::new();
    let ic_to_copro1_ready = Signal::new();
    let copro1_to_ic_ready = Signal::new();

    let ic_to_copro2 = Signal::new();
    let copro2_to_ic = Signal::new();
    let ic_to_copro2_ready = Signal::new();
    let copro2_to_ic_ready = Signal::new();

    let ic_to_copro3 = Signal::new();
    let copro3_to_ic = Signal::new();
    let ic_to_copro3_ready = Signal::new();
    let copro3_to_ic_ready = Signal::new();

    let children = vec![
        task::spawn(async move {
            let mut packet_gen_ports = packet_gen::Ports {
                pro_to_ic: ports::Out::connect(pro_to_ic.tx),
                ic_to_pro: ports::In::connect(ic_to_pro.rx),
            };
            packet_gen::process(&mut packet_gen_ports).await;
        }),
        task::spawn(async move {
            let mut interconnect_ports = interconnect::Ports {
                pro_to_ic: ports::In::connect(pro_to_ic.rx),
                ic_to_pro: ports::Out::connect(ic_to_pro.tx),

                ic_to_copro1_ready: ports::Out::connect(ic_to_copro1_ready.tx),
                ic_to_copro1: ports::Out::connect(ic_to_copro1.tx),
                copro1_to_ic_ready: ports::In::connect(copro1_to_ic_ready.rx),
                copro1_to_ic: ports::In::connect(copro1_to_ic.rx),

                ic_to_copro2_ready: ports::Out::connect(ic_to_copro2_ready.tx),
                ic_to_copro2: ports::Out::connect(ic_to_copro2.tx),
                copro2_to_ic_ready: ports::In::connect(copro2_to_ic_ready.rx),
                copro2_to_ic: ports::In::connect(copro2_to_ic.rx),

                ic_to_copro3_ready: ports::Out::connect(ic_to_copro3_ready.tx),
                ic_to_copro3: ports::Out::connect(ic_to_copro3.tx),
                copro3_to_ic_ready: ports::In::connect(copro3_to_ic_ready.rx),
                copro3_to_ic: ports::In::connect(copro3_to_ic.rx),
            };
            interconnect::process(&mut interconnect_ports).await;
        }),
        task::spawn(async move {
            let mut copro1_ports = copro1::Ports {
                ic_to_copro1_ready: ports::In::connect(ic_to_copro1_ready.rx),
                ic_to_copro1: ports::In::connect(ic_to_copro1.rx),
                copro1_to_ic_ready: ports::Out::connect(copro1_to_ic_ready.tx),
                copro1_to_ic: ports::Out::connect(copro1_to_ic.tx),
            };
            copro1::process(&mut copro1_ports).await;
        }),
        task::spawn(async move {
            let mut copro2 = copro2::Ports {
                ic_to_copro2_ready: ports::In::connect(ic_to_copro2_ready.rx),
                ic_to_copro2: ports::In::connect(ic_to_copro2.rx),
                copro2_to_ic_ready: ports::Out::connect(copro2_to_ic_ready.tx),
                copro2_to_ic: ports::Out::connect(copro2_to_ic.tx),
            };
            copro2::process(&mut copro2).await;
        }),
        task::spawn(async move {
            let mut copro3 = copro3::Ports {
                ic_to_copro3_ready: ports::In::connect(ic_to_copro3_ready.rx),
                ic_to_copro3: ports::In::connect(ic_to_copro3.rx),
                copro3_to_ic_ready: ports::Out::connect(copro3_to_ic_ready.tx),
                copro3_to_ic: ports::Out::connect(copro3_to_ic.tx),
            };
            copro3::process(&mut copro3).await;
        }),
    ];

    // Wait for the tasks to complete any remaining work
    join_all(children).await;
}
