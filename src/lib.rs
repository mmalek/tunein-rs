extern crate xml;
use std::io::Read;

// pub struct Reader<R: Read>
// {
//     reader: xml::reader::EventReader<R>
// }

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
    pub status : u32,
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
        Head{title: String::new(), status: 0}
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

// impl<R: Read> Reader<R>
// {
//     pub fn new(source: R) -> Reader<R>
//     {
//         Reader{reader: xml::reader::EventReader::new(source)}
//     }
    
//     pub fn next(&mut self) -> Result<Event, xml::reader::Error>
//     {
//         self.reader.next().and_then(|event| {
//             match event
//             {
//                 xml::reader::XmlEvent::StartElement(ref name, ref attr, _) => if name.local_name == "opml"{ Event::Link{text: "aa", url: "bb", key: "cc"} } else {},
//                 _ => {}
//             }
//         });
//     }
// }

pub fn parse<R: Read>(source: R) -> Document {
    
    let mut document = Document::new();
    
    // let reader = Reader::new(source);
    let mut reader = xml::reader::EventReader::new(source);
    
    loop {
        if let Ok(event) = reader.next() {
            match event
            {
                xml::reader::XmlEvent::StartElement{ref name, ref attributes, ..} => {
                    match &name.local_name as &str {
                        "opml" => {
                            if let Some(version) = attributes.iter().find(|ref attr| attr.name.local_name == "version").and_then(|v| v.value.parse::<u8>().ok()) {
                                document.version.major = version;
                            }
                        }
                        "outline" => {
                            if let Some(otype) = attributes.iter().find(|ref attr| attr.name.local_name == "type") {
                                if otype.value == "link" {
                                    let mut link = Link::new(); 
                                    for attr in attributes {
                                        match &attr.name.local_name as &str {
                                            "text" => link.text = attr.value.clone(),
                                            "URL" => link.url = attr.value.clone(),
                                            "key" => link.key = attr.value.clone(),
                                            _ => {}
                                        }
                                    }
                                    document.outlines.push(Outline::Link(link));
                                }
                                else if otype.value == "audio" {
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
                                    document.outlines.push(Outline::Audio(audio));
                                }
                            }
                        }
                        _ => {}
                    }
                }
                xml::reader::XmlEvent::EndDocument => { break; }
                _ => {}
            }
        }
        else {
            break;
        }
    }
    
    return document;
}

