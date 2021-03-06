use crate::common::{Audio, Format, Link};
use crate::error::Error;
use crate::event::{Event, OutlineEvent};
use std::io::Read;
use std::iter::{IntoIterator, Iterator};

pub struct Reader<R: Read> {
    reader: xml::reader::EventReader<R>,
}

impl<R: Read> Reader<R> {
    pub fn new(source: R) -> Reader<R> {
        Reader {
            reader: xml::reader::EventReader::new(source),
        }
    }

    pub fn next(&mut self) -> Result<Event, Error> {
        let mut content = String::new();
        loop {
            match self.reader.next()? {
                xml::reader::XmlEvent::StartElement {
                    ref name,
                    ref attributes,
                    ..
                } => match name.local_name.as_str() {
                    "head" => {
                        return Ok(Event::StartHead);
                    }
                    "body" => {
                        return Ok(Event::StartBody);
                    }
                    "title" | "status" => {}
                    "opml" => {
                        return parse_opml(&attributes);
                    }
                    "outline" => {
                        return parse_outline(&attributes);
                    }
                    _ => {
                        return Err(Error::UnexpectedElement);
                    }
                },
                xml::reader::XmlEvent::Characters(s) => {
                    content = s;
                }
                xml::reader::XmlEvent::EndElement { ref name } => {
                    return match name.local_name.as_str() {
                        "head" => Ok(Event::EndHead),
                        "body" => Ok(Event::EndBody),
                        "title" => Ok(Event::Title(content)),
                        "status" => Ok(Event::Status(content.parse().ok())),
                        "opml" => Ok(Event::EndDocument),
                        "outline" => Ok(Event::EndOutline),
                        _ => Err(Error::UnexpectedElement),
                    }
                }
                xml::reader::XmlEvent::EndDocument => {}
                _ => {}
            }
        }
    }
}

fn parse_opml(attributes: &[xml::attribute::OwnedAttribute]) -> Result<Event, Error> {
    attributes
        .iter()
        .find(|ref attr| attr.name.local_name == "version")
        .ok_or(Error::MissingVersionAttr)
        .and_then(|v| {
            v.value
                .parse::<u8>()
                .map_err(|_| Error::InvalidVersionFormat)
        })
        .map(|version| Event::StartDocument { version })
}

fn parse_outline(attributes: &[xml::attribute::OwnedAttribute]) -> Result<Event, Error> {
    attributes
        .iter()
        .find(|ref attr| attr.name.local_name == "type")
        .map_or_else(
            || parse_group(attributes),
            |outline_type| match outline_type.value.as_str() {
                "link" => parse_link(attributes),
                "audio" => parse_audio(attributes),
                "text" => parse_text(attributes),
                _ => Err(Error::InvalidOutlineType),
            },
        )
        .map(Box::new)
        .map(Event::StartOutline)
}

fn parse_group(attributes: &[xml::attribute::OwnedAttribute]) -> Result<OutlineEvent, Error> {
    let mut text = String::new();
    let mut key = String::new();
    for attr in attributes {
        match attr.name.local_name.as_str() {
            "text" => text = attr.value.clone(),
            "key" => key = attr.value.clone(),
            _ => {}
        }
    }
    Ok(OutlineEvent::Group { text, key })
}

fn parse_link(attributes: &[xml::attribute::OwnedAttribute]) -> Result<OutlineEvent, Error> {
    let mut link = Link::default();
    for attr in attributes {
        match attr.name.local_name.as_str() {
            "text" => link.text = attr.value.clone(),
            "URL" => link.url = attr.value.clone(),
            "key" => link.key = attr.value.clone(),
            "guide_id" => link.guide_id = attr.value.clone(),
            _ => {}
        }
    }
    Ok(OutlineEvent::Link(link))
}

fn parse_audio(attributes: &[xml::attribute::OwnedAttribute]) -> Result<OutlineEvent, Error> {
    let mut audio = Audio::default();
    for attr in attributes {
        match attr.name.local_name.as_str() {
            "text" => audio.text = attr.value.clone(),
            "subtext" => audio.subtext = attr.value.clone(),
            "URL" => audio.url = attr.value.clone(),
            "bitrate" => {
                audio.bitrate = attr
                    .value
                    .parse()
                    .map_err(|_| Error::InvalidBitrateFormat)?
            }
            "reliability" => {
                audio.reliability = attr
                    .value
                    .parse()
                    .map_err(|_| Error::InvalidReliabilityFormat)?
            }
            "formats" => {
                audio.format = match attr.value.as_str() {
                    "mp3" => Format::MP3,
                    _ => Format::Unknown,
                }
            }
            "item" => audio.item = attr.value.clone(),
            "image" => audio.image = attr.value.clone(),
            "guide_id" => audio.guide_id = attr.value.clone(),
            "genre_id" => audio.genre_id = attr.value.clone(),
            "now_playing_id" => audio.now_playing_id = attr.value.clone(),
            "preset_id" => audio.preset_id = attr.value.clone(),
            _ => {}
        }
    }
    Ok(OutlineEvent::Audio(audio))
}

fn parse_text(attributes: &[xml::attribute::OwnedAttribute]) -> Result<OutlineEvent, Error> {
    let text = attributes
        .iter()
        .find(|&attr| attr.name.local_name == "text")
        .map(|attr| attr.value.clone())
        .unwrap_or_default();
    Ok(OutlineEvent::Text(text))
}

impl<R: Read> IntoIterator for Reader<R> {
    type Item = Result<Event, Error>;
    type IntoIter = Events<R>;

    fn into_iter(self) -> Events<R> {
        Events {
            reader: self,
            finished: false,
        }
    }
}

pub struct Events<R: Read> {
    reader: Reader<R>,
    finished: bool,
}

impl<R: Read> Iterator for Events<R> {
    type Item = Result<Event, Error>;

    fn next(&mut self) -> Option<Result<Event, Error>> {
        if self.finished {
            None
        } else {
            let result = self.reader.next();
            self.finished = matches!(result, Ok(Event::EndDocument) | Err(_));
            Some(result)
        }
    }
}
