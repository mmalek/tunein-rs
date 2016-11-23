extern crate xml;

use common::*;
use reader::event::*;
use std::io::Read;
use std::error::Error as StdError;
use std::iter::{Iterator, IntoIterator};

pub struct Reader<R: Read> {
    reader: xml::reader::EventReader<R>,
}

fn map_err(e: xml::reader::Error) -> Error {
    Error::new(e.description())
}

impl<R: Read> Reader<R> {
    pub fn new(source: R) -> Reader<R> {
        Reader { reader: xml::reader::EventReader::new(source) }
    }

    pub fn next(&mut self) -> Result<Event> {
        let mut content = String::new();
        loop {
            match self.reader.next().map_err(map_err)? {
                xml::reader::XmlEvent::StartElement { ref name, ref attributes, .. } => {
                    match &name.local_name as &str {
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
                            return Err(Error::new("Unexpected element"));
                        }
                    }
                }
                xml::reader::XmlEvent::Characters(s) => {
                    content = s;
                }
                xml::reader::XmlEvent::EndElement { ref name } => {
                    return match &name.local_name as &str {
                        "head" => Ok(Event::EndHead),
                        "body" => Ok(Event::EndBody),
                        "title" => Ok(Event::Title(content)),
                        "status" => Ok(Event::Status(content.parse().ok())),
                        "opml" => Ok(Event::EndDocument),
                        "outline" => Ok(Event::EndOutline),
                        _ => Err(Error::new("Unexpected element")),
                    }
                }
                xml::reader::XmlEvent::EndDocument => {}
                _ => {}
            }
        }
    }
}

fn parse_opml(attributes: &Vec<xml::attribute::OwnedAttribute>) -> Result<Event> {
    attributes.iter()
        .find(|ref attr| attr.name.local_name == "version")
        .ok_or(Error::new("Missing version attribute"))
        .and_then(|v| v.value.parse::<u8>().map_err(|_| Error::new("Invalid version format")))
        .and_then(|version| Ok(Event::StartDocument { version: version }))
}

fn parse_outline(attributes: &Vec<xml::attribute::OwnedAttribute>) -> Result<Event> {
    attributes.iter()
        .find(|ref attr| attr.name.local_name == "type")
        .map_or_else(|| parse_group(attributes), |outline_type| {
            match &outline_type.value as &str {
                "link" => parse_link(attributes),
                "audio" => parse_audio(attributes),
                _ => Err(Error::new("Invalid outline type")),
            }
        })
        .map(|outline| Event::StartOutline(outline))
}

fn parse_group(attributes: &Vec<xml::attribute::OwnedAttribute>) -> Result<OutlineEvent> {
    let mut text = String::new();
    let mut key = String::new();
    for attr in attributes {
        match &attr.name.local_name as &str {
            "text" => text = attr.value.clone(),
            "key" => key = attr.value.clone(),
            _ => {}
        }
    }
    Ok(OutlineEvent::Group {
        text: text,
        key: key,
    })
}

fn parse_link(attributes: &Vec<xml::attribute::OwnedAttribute>) -> Result<OutlineEvent> {
    let mut link = Link::new();
    for attr in attributes {
        match &attr.name.local_name as &str {
            "text" => link.text = attr.value.clone(),
            "URL" => link.url = attr.value.clone(),
            "key" => link.key = attr.value.clone(),
            _ => {}
        }
    }
    Ok(OutlineEvent::Link(link))
}

fn parse_audio(attributes: &Vec<xml::attribute::OwnedAttribute>) -> Result<OutlineEvent> {
    let mut audio = Audio::new();
    for attr in attributes {
        match &attr.name.local_name as &str {
            "text" => audio.text = attr.value.clone(),
            "subtext" => audio.subtext = attr.value.clone(),
            "URL" => audio.url = attr.value.clone(),
            "bitrate" => {
                audio.bitrate = attr.value
                    .parse()
                    .map_err(|_| Error::new("Invalid bitrate format"))?
            }
            "reliability" => {
                audio.reliability = attr.value
                    .parse()
                    .map_err(|_| Error::new("Invalid reliability format"))?
            }
            "formats" => {
                audio.format = match &attr.value as &str {
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

impl<R: Read> IntoIterator for Reader<R> {
    type Item = Result<Event>;
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
    type Item = Result<Event>;

    fn next(&mut self) -> Option<Result<Event>> {
        if self.finished {
            None
        } else {
            let result = self.reader.next();
            self.finished = match result {
                Ok(Event::EndDocument) |
                Err(_) => true,
                _ => false,
            };
            Some(result)
        }
    }
}
