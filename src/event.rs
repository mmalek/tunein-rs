use crate::common::{Audio, Group, Link, Outline};

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
    StartOutline(Box<OutlineEvent>),
    EndOutline,
}

#[derive(Debug, PartialEq)]
pub enum OutlineEvent {
    Group { text: String, key: String },
    Link(Link),
    Audio(Audio),
    Text(String),
}

impl From<OutlineEvent> for Outline {
    fn from(outline: OutlineEvent) -> Outline {
        match outline {
            OutlineEvent::Group { text, key } => Outline::Group(Group {
                text,
                key,
                outlines: vec![],
            }),
            OutlineEvent::Link(link) => Outline::Link(link),
            OutlineEvent::Audio(audio) => Outline::Audio(audio),
            OutlineEvent::Text(text) => Outline::Text(text),
        }
    }
}
