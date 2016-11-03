extern crate xml;

use common::*;
use reader::event::*;
use std::io::Read;
use std::error::Error as StdError;
use std::iter::{Iterator, IntoIterator};

#[derive(Debug, PartialEq)]
enum Node
{
    Opml,
    Head,
    Title,
    Status,
    Body,
    Outline{is_group: bool},
}

impl Node
{
    fn get_name(&self) -> &'static str
    {
        match *self {
            Node::Opml => { "opml" }
            Node::Head => { "head" }
            Node::Title => { "title" }
            Node::Status => { "status" }
            Node::Body => { "body" }
            Node::Outline{..} => { "outline" }
        }
    }
}

pub struct Reader<R: Read>
{
    reader: xml::reader::EventReader<R>,
    path: Vec<Node>,
}

fn map_err(e: xml::reader::Error) -> Error
{
    Error{description: e.description().to_string()}
}

impl<R: Read> Reader<R>
{
    pub fn new(source: R) -> Reader<R>
    {
        Reader{reader: xml::reader::EventReader::new(source), path: vec![]}
    }

    pub fn next(&mut self) -> Result<Event>
    {
        let mut content = String::new();
        loop
        {
            match try!(self.reader.next().map_err(map_err))
            {
                xml::reader::XmlEvent::StartElement{ref name, ref attributes, ..} => {
                    match &name.local_name as &str {
                        "head" => {
                            self.path.push(Node::Head);
                            return Ok(Event::StartHead);
                        }
                        "body" => {
                            self.path.push(Node::Body);
                            return Ok(Event::StartBody);
                        }
                        "title" => {
                            self.path.push(Node::Title);
                        }
                        "status" => {
                            self.path.push(Node::Status);
                        }
                        "opml" => {
                            if let Some(version) = attributes.iter().find(|ref attr| attr.name.local_name == "version").and_then(|v| v.value.parse::<u8>().ok()) {
                                self.path.push(Node::Opml);
                                return Ok(Event::StartDocument{version: version});
                            } else {
                                return Err(Error{description: "Invalid version format".to_string()});
                            }
                        }
                        "outline" => {
                            if let Some(outline_type) = attributes.iter().find(|ref attr| attr.name.local_name == "type") {
                                self.path.push(Node::Outline{is_group: false});
                                if outline_type.value == "link" {
                                    let mut link = Link::new();
                                    for attr in attributes {
                                        match &attr.name.local_name as &str {
                                            "text" => link.text = attr.value.clone(),
                                            "URL" => link.url = attr.value.clone(),
                                            "key" => link.key = attr.value.clone(),
                                            _ => {}
                                        }
                                    }
                                    return Ok(Event::Link(link));
                                }
                                else if outline_type.value == "audio" {
                                    let mut audio = Audio::new();
                                    for attr in attributes {
                                        match &attr.name.local_name as &str {
                                            "text" => audio.text = attr.value.clone(),
                                            "subtext" => audio.subtext = attr.value.clone(),
                                            "URL" => audio.url = attr.value.clone(),
                                            "bitrate" => if let Ok(bitrate) = attr.value.parse() { audio.bitrate = bitrate; },
                                            "reliability" => if let Ok(reliability) = attr.value.parse() { audio.reliability = reliability; },
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
                                    return Ok(Event::Audio(audio));
                                } else {
                                    return Err(Error{description: "Invalid outline type".to_string()});
                                }
                            } else {
                                self.path.push(Node::Outline{is_group: true});
                                return Ok(Event::StartOutlineGroup{
                                    text: attributes.iter().find(|attr| attr.name.local_name == "text").map_or(String::new(), |attr| attr.value.clone()),
                                    key: attributes.iter().find(|attr| attr.name.local_name == "key").map_or(String::new(), |attr| attr.value.clone()),
                                });
                            }
                        }
                        _ => { return Err(Error{description: "Unexpected element".to_string()}); }
                    }
                }
                xml::reader::XmlEvent::Characters(s) => { content = s; }
                xml::reader::XmlEvent::EndElement{ref name} => {
                    if self.path.last().and_then(|node| Some(node.get_name())) == Some(&name.local_name) {
                        if let Some(node) = self.path.pop() {
                            match node {
                                Node::Opml => { return Ok(Event::EndDocument); }
                                Node::Head => { return Ok(Event::EndHead); }
                                Node::Title => { return Ok(Event::Title(content.drain(..).collect::<String>())); }
                                Node::Status => { return Ok(Event::Status(content.drain(..).collect::<String>().parse().ok())); }
                                Node::Body => { return Ok(Event::EndBody); }
                                Node::Outline{is_group: true} => { return Ok(Event::EndOutlineGroup); }
                                Node::Outline{is_group: false} => {}
                            }
                        }
                    }
                }
                xml::reader::XmlEvent::EndDocument => {
                    if !self.path.is_empty() {
                        return Err(Error{description: "Unexpected end of the document".to_string()});
                    }
                }
                _ => {}
            }
        }
    }
}

impl<R: Read> IntoIterator for Reader<R> {
    type Item = Result<Event>;
    type IntoIter = Events<R>;

    fn into_iter(self) -> Events<R> {
        Events{reader: self, finished: false}
    }
}

pub struct Events<R: Read> {
    reader: Reader<R>,
    finished: bool,
}

impl<R: Read> Iterator for Events<R> {
    type Item = Result<Event>;

    fn next(&mut self) -> Option<Result<Event>>{
        if self.finished {
            None
        } else {
            let result = self.reader.next();
            self.finished = match result {
                Ok(Event::EndDocument) | Err(_) => true,
                _ => false,
            };
            Some(result)
        }
    }
}
