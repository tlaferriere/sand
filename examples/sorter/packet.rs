#[derive(Clone, PartialEq)]
pub struct Packet {
    pub(crate) id: u32,
    pub(crate) address: u32,
    pub(crate) payload: Vec<u32>,
    pub(crate) payload_size: u32,
}
