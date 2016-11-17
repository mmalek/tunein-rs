
use common::*;

pub enum Event {
    StartDocument { version: u8 },
    EndDocument,
    StartHead,
    EndHead,
    StartBody,
    EndBody,
    Title(String),
    Status(Option<u32>),
    StartOutline(Outline),
    EndOutline,
}
