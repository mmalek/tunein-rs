use crate::common::*;

#[derive(Debug, PartialEq)]
pub enum Event {
    StartDocument { version: u8 },
    EndDocument,
    StartHead,
    EndHead,
    StartBody,
    EndBody,
    Title(String),
    Status(Option<u32>),
    StartOutline(OutlineEvent),
    EndOutline,
}

#[derive(Debug, PartialEq)]
pub enum OutlineEvent {
    Group { text: String, key: String },
    Link(Link),
    Audio(Audio),
}
