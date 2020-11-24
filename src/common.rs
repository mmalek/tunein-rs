use crate::event::*;
use std::error::Error as StdError;
use std::fmt;
use std::result;

#[derive(Debug, PartialEq)]
pub struct Document {
    pub version: Version,
    pub head: Head,
    pub outlines: Vec<Outline>,
}

#[derive(Debug, PartialEq)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
}

#[derive(Debug, PartialEq)]
pub struct Head {
    pub title: String,
    pub status: Option<u32>,
}

#[derive(Debug, PartialEq)]
pub struct Group {
    pub text: String,
    pub key: String,
    pub outlines: Vec<Outline>,
}

#[derive(Debug, PartialEq)]
pub struct Link {
    pub text: String,
    pub url: String,
    pub key: String,
    pub guide_id: String,
}

#[derive(Debug, PartialEq)]
pub struct Audio {
    pub text: String,
    pub subtext: String,
    pub url: String,
    pub bitrate: u16,
    pub reliability: u16,
    pub format: Format,
    pub item: String,
    pub image: String,
    pub guide_id: String,
    pub genre_id: String,
    pub now_playing_id: String,
    pub preset_id: String,
}

#[derive(Debug, PartialEq)]
pub enum Outline {
    Group(Group),
    Link(Link),
    Audio(Audio),
}

#[derive(Debug, PartialEq)]
pub enum Format {
    Unknown,
    MP3,
}

impl Document {
    pub fn new() -> Document {
        Document {
            version: Version::new(),
            head: Head::new(),
            outlines: Vec::new(),
        }
    }
}

impl Version {
    pub fn new() -> Version {
        Version { major: 0, minor: 0 }
    }
}

impl Head {
    pub fn new() -> Head {
        Head {
            title: String::new(),
            status: None,
        }
    }
}

impl Group {
    pub fn new() -> Group {
        Group {
            text: String::new(),
            key: String::new(),
            outlines: vec![],
        }
    }
}

impl Link {
    pub fn new() -> Link {
        Link {
            text: String::new(),
            url: String::new(),
            key: String::new(),
            guide_id: String::new(),
        }
    }
}

impl Audio {
    pub fn new() -> Audio {
        Audio {
            text: String::new(),
            subtext: String::new(),
            url: String::new(),
            bitrate: 0,
            reliability: 0,
            format: Format::Unknown,
            item: String::new(),
            image: String::new(),
            guide_id: String::new(),
            genre_id: String::new(),
            now_playing_id: String::new(),
            preset_id: String::new(),
        }
    }
}

impl From<OutlineEvent> for Outline {
    fn from(outline: OutlineEvent) -> Outline {
        match outline {
            OutlineEvent::Group { text, key } => Outline::Group(Group {
                text: text,
                key: key,
                outlines: vec![],
            }),
            OutlineEvent::Link(link) => Outline::Link(link),
            OutlineEvent::Audio(audio) => Outline::Audio(audio),
        }
    }
}

#[derive(Debug)]
pub struct Error {
    pub description: String,
}

impl Error {
    pub fn new(description: &str) -> Error {
        Error {
            description: description.into(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.description())
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        self.description.as_str()
    }
    fn cause(&self) -> Option<&StdError> {
        None
    }
}

pub type Result<T> = result::Result<T, Error>;
