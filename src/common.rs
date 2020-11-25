#[derive(Debug, Default, PartialEq)]
pub struct Document {
    pub version: Version,
    pub head: Head,
    pub outlines: Vec<Outline>,
}

#[derive(Debug, Default, PartialEq)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
}

#[derive(Debug, Default, PartialEq)]
pub struct Head {
    pub title: String,
    pub status: Option<u32>,
}

#[derive(Debug, Default, PartialEq)]
pub struct Group {
    pub text: String,
    pub key: String,
    pub outlines: Vec<Outline>,
}

#[derive(Debug, Default, PartialEq)]
pub struct Link {
    pub text: String,
    pub url: String,
    pub key: String,
    pub guide_id: String,
}

#[derive(Debug, Default, PartialEq)]
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

impl Default for Format {
    fn default() -> Format {
        Format::Unknown
    }
}
