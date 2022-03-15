use futures::future::join_all;
use system_rust::signal::signal;
use system_rust::SignalWrite;
use system_rust_macros::connections;

mod copro1;
mod copro2;
mod copro3;
mod interconnect;
mod packet;
mod packet_gen;

#[tokio::main]
async fn main() {
    let pro_to_ic = signal();
    let ic_to_pro = signal();

    let ic_to_copro1 = signal();
    let copro1_to_ic = signal();
    let ic_to_copro1_ready = signal();
    let copro1_to_ic_ready = signal();

    let ic_to_copro2 = signal();
    let copro2_to_ic = signal();
    let ic_to_copro2_ready = signal();
    let copro2_to_ic_ready = signal();

    let ic_to_copro3 = signal();
    let copro3_to_ic = signal();
    let ic_to_copro3_ready = signal();
    let copro3_to_ic_ready = signal();

    connections! {
            packet_gen.pro_to_ic -> pro_to_ic;
            packet_gen.ic_to_pro <- ic_to_pro;
            interconnect.pro_to_ic <- pro_to_ic;
            interconnect.ic_to_pro -> ic_to_pro;
            interconnect.ic_to_copro1_ready -> ic_to_copro1_ready;
            interconnect.ic_to_copro1 -> ic_to_copro1;
            interconnect.copro1_to_ic_ready <- copro1_to_ic_ready;
            interconnect.copro1_to_ic <- copro1_to_ic;
            interconnect.ic_to_copro2_ready -> ic_to_copro2_ready;
            interconnect.ic_to_copro2 -> ic_to_copro2;
            interconnect.copro2_to_ic_ready <- copro2_to_ic_ready;
            interconnect.copro2_to_ic <- copro2_to_ic;
            interconnect.ic_to_copro3_ready -> ic_to_copro3_ready;
            interconnect.ic_to_copro3 -> ic_to_copro3;
            interconnect.copro3_to_ic_ready <- copro3_to_ic_ready;
            interconnect.copro3_to_ic <- copro3_to_ic;
            copro1.ic_to_copro1_ready <- ic_to_copro1_ready;
            copro1.ic_to_copro1 <- ic_to_copro1;
            copro1.copro1_to_ic_ready -> copro1_to_ic_ready;
            copro1.copro1_to_ic -> copro1_to_ic;
            copro2.ic_to_copro2_ready <- ic_to_copro2_ready;
            copro2.ic_to_copro2 <- ic_to_copro2;
            copro2.copro2_to_ic_ready -> copro2_to_ic_ready;
            copro2.copro2_to_ic -> copro2_to_ic;
            copro3.ic_to_copro3_ready <- ic_to_copro3_ready;
            copro3.ic_to_copro3 <- ic_to_copro3;
            copro3.copro3_to_ic_ready -> copro3_to_ic_ready;
            copro3.copro3_to_ic -> copro3_to_ic;
    }
}
