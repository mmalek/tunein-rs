extern crate xml;
use std::io::Read;
use std::error::Error as StdError;

struct Reader<R: Read>
{
    reader: xml::reader::EventReader<R>,
    path : Vec<Node>,
}

enum Event
{
    StartDocument{version: u8},
    EndDocument,
    StartHead,
    EndHead,
    StartBody,
    EndBody,
    Title(String),
    Status(Option<u32>),
    StartOutlineGroup{text: String, key: String},
    EndOutlineGroup,
    Link(Link),
    Audio(Audio),
}

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

#[derive(Debug, PartialEq)]
pub struct Document
{
    pub version : Version,
    pub head : Head,
    pub outlines : Vec<Outline>,
}

#[derive(Debug, PartialEq)]
pub struct Version
{
    pub major : u8,
    pub minor : u8,
}

#[derive(Debug, PartialEq)]
pub struct Head
{
    pub title : String,
    pub status : Option<u32>,
}

#[derive(Debug, PartialEq)]
pub struct Link {
    pub text : String,
    pub url : String,
    pub key : String,
}

#[derive(Debug, PartialEq)]
pub struct Audio {
    pub text : String,
    pub subtext : String,
    pub url : String,
    pub bitrate : u16,
    pub reliability : u16,
    pub format : Format,
    pub item : String,
    pub image : String,
    pub guide_id : String,
    pub genre_id : String,
    pub now_playing_id : String,
    pub preset_id : String,
}

#[derive(Debug, PartialEq)]
pub enum Outline
{
    Group{text: String, key: String, outlines: Vec<Outline>},
    Link(Link),
    Audio(Audio),
}

#[derive(Debug, PartialEq)]
pub enum Format
{
    Unknown,
    MP3
}

impl Document
{
    pub fn new() -> Document
    {
        Document{version: Version::new(), head: Head::new(), outlines: Vec::new() }
    }
}

impl Version
{
    pub fn new() -> Version
    {
        Version{major: 0, minor: 0}
    }
}

impl Head
{
    pub fn new() -> Head
    {
        Head{title: String::new(), status: None}
    }
}

impl Link
{
    pub fn new() -> Link
    {
        Link{text: String::new(), url: String::new(), key: String::new()}
    }
}

impl Audio
{
    pub fn new() -> Audio
    {
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
            preset_id: String::new()
        }
    }
}

#[derive(Debug)]
pub struct Error
{
    description : String,
}

pub type Result<T> = std::result::Result<T, Error>;

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

pub fn parse<R: Read>(source: R) -> Result<Document>  {

    let mut document = Document::new();

    let mut reader = Reader::new(source);

    let mut outline_groups: Vec<Vec<Outline>> = vec![];

    loop {
        match try!(reader.next())
        {
            Event::StartDocument{version} => {
                document.version.major = version;
                outline_groups.push(vec![]);
                println!("Start document");
            },
            Event::EndDocument => {
                println!("End document");
                if let Some(outlines) = outline_groups.pop() {
                    document.outlines = outlines;
                }
                break;
            },
            Event::Title(title) => {
                document.head.title = title;
            },
            Event::Status(status) => {
                document.head.status = status;
            },
            Event::Link(link) => {
                if let Some(ref mut outlines) = outline_groups.last_mut() {
                    println!("Got link: {:?}", &link);
                    outlines.push(Outline::Link(link));
                }
            },
            Event::Audio(audio) => {
                println!("Before audio");
                if let Some(ref mut outlines) = outline_groups.last_mut() {
                    outlines.push(Outline::Audio(audio));
                }
            },
            Event::StartOutlineGroup{text, key} => {
                println!("Before new outline group");
                if let Some(ref mut outlines) = outline_groups.last_mut() {
                    outlines.push(Outline::Group{text: text, key: key, outlines: vec![]});
                }
                outline_groups.push(vec![]);
            },
            Event::EndOutlineGroup => {
                println!("Before end outline group");
                if let Some(ref mut children) = outline_groups.pop() {
                    match outline_groups.last_mut().and_then(|o| o.last_mut()) {
                        Some(&mut Outline::Group{ref mut outlines, ..}) => { outlines.append(children); }
                        Some(_) => {}
                        None => { return Err(Error{description: "End/start elements doesn't match".to_string()}); }
                    }
                }
            },
            _ => {},
        }
    }

    return Ok(document);
}
