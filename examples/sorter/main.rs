use futures::future::join_all;
use system_rust::ports;
use system_rust::signal::signal;
use tokio::task;

mod copro1;
mod copro2;
mod copro3;
mod interconnect;
mod packet;
mod packet_gen;

#[tokio::main]
async fn main() {
    let (pro_to_ic_tx, pro_to_ic_rx) = signal();
    let (ic_to_pro_tx, ic_to_pro_rx) = signal();

    let (ic_to_copro1_tx, ic_to_copro1_rx) = signal();
    let (copro1_to_ic_tx, copro1_to_ic_rx) = signal();
    let (ic_to_copro1_ready_tx, ic_to_copro1_ready_rx) = signal();
    let (copro1_to_ic_ready_tx, copro1_to_ic_ready_rx) = signal();

    let (ic_to_copro2_tx, ic_to_copro2_rx) = signal();
    let (copro2_to_ic_tx, copro2_to_ic_rx) = signal();
    let (ic_to_copro2_ready_tx, ic_to_copro2_ready_rx) = signal();
    let (copro2_to_ic_ready_tx, copro2_to_ic_ready_rx) = signal();

    let (ic_to_copro3_tx, ic_to_copro3_rx) = signal();
    let (copro3_to_ic_tx, copro3_to_ic_rx) = signal();
    let (ic_to_copro3_ready_tx, ic_to_copro3_ready_rx) = signal();
    let (copro3_to_ic_ready_tx, copro3_to_ic_ready_rx) = signal();

    let children = vec![
        task::spawn(async move {
            let mut packet_gen_ports = packet_gen::Ports {
                pro_to_ic: ports::Out::connect(pro_to_ic_tx),
                ic_to_pro: ports::In::connect(ic_to_pro_rx),
            };
            packet_gen::process(&mut packet_gen_ports).await;
        }),
        task::spawn(async move {
            let mut interconnect_ports = interconnect::Ports {
                pro_to_ic: ports::In::connect(pro_to_ic_rx),
                ic_to_pro: ports::Out::connect(ic_to_pro_tx),

                ic_to_copro1_ready: ports::Out::connect(ic_to_copro1_ready_tx),
                ic_to_copro1: ports::Out::connect(ic_to_copro1_tx),
                copro1_to_ic_ready: ports::In::connect(copro1_to_ic_ready_rx),
                copro1_to_ic: ports::In::connect(copro1_to_ic_rx),

                ic_to_copro2_ready: ports::Out::connect(ic_to_copro2_ready_tx),
                ic_to_copro2: ports::Out::connect(ic_to_copro2_tx),
                copro2_to_ic_ready: ports::In::connect(copro2_to_ic_ready_rx),
                copro2_to_ic: ports::In::connect(copro2_to_ic_rx),

                ic_to_copro3_ready: ports::Out::connect(ic_to_copro3_ready_tx),
                ic_to_copro3: ports::Out::connect(ic_to_copro3_tx),
                copro3_to_ic_ready: ports::In::connect(copro3_to_ic_ready_rx),
                copro3_to_ic: ports::In::connect(copro3_to_ic_rx),
            };
            interconnect::process(&mut interconnect_ports).await;
        }),
        task::spawn(async move {
            let mut copro1_ports = copro1::Ports {
                ic_to_copro1_ready: ports::In::connect(ic_to_copro1_ready_rx),
                ic_to_copro1: ports::In::connect(ic_to_copro1_rx),
                copro1_to_ic_ready: ports::Out::connect(copro1_to_ic_ready_tx),
                copro1_to_ic: ports::Out::connect(copro1_to_ic_tx),
            };
            copro1::process(&mut copro1_ports).await;
        }),
        task::spawn(async move {
            let mut copro2 = copro2::Ports {
                ic_to_copro2_ready: ports::In::connect(ic_to_copro2_ready_rx),
                ic_to_copro2: ports::In::connect(ic_to_copro2_rx),
                copro2_to_ic_ready: ports::Out::connect(copro2_to_ic_ready_tx),
                copro2_to_ic: ports::Out::connect(copro2_to_ic_tx),
            };
            copro2::process(&mut copro2).await;
        }),
        task::spawn(async move {
            let mut copro3 = copro3::Ports {
                ic_to_copro3_ready: ports::In::connect(ic_to_copro3_ready_rx),
                ic_to_copro3: ports::In::connect(ic_to_copro3_rx),
                copro3_to_ic_ready: ports::Out::connect(copro3_to_ic_ready_tx),
                copro3_to_ic: ports::Out::connect(copro3_to_ic_tx),
            };
            copro3::process(&mut copro3).await;
        }),
    ];

    // Wait for the tasks to complete any remaining work
    join_all(children).await;
}
