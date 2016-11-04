use common::*;
use reader::*;

use std::io::Read;

pub fn read<R: Read>(source: R) -> Result<Document> {

    let mut document = Document::new();

    let mut outline_groups: Vec<Vec<Outline>> = vec![];

    for event in Reader::new(source) {
        match try!(event) {
            Event::StartDocument { version } => {
                document.version.major = version;
                outline_groups.push(vec![]);
            }
            Event::EndDocument => {
                if let Some(outlines) = outline_groups.pop() {
                    document.outlines = outlines;
                }
            }
            Event::Title(title) => {
                document.head.title = title;
            }
            Event::Status(status) => {
                document.head.status = status;
            }
            Event::Link(link) => {
                if let Some(ref mut outlines) = outline_groups.last_mut() {
                    outlines.push(Outline::Link(link));
                }
            }
            Event::Audio(audio) => {
                if let Some(ref mut outlines) = outline_groups.last_mut() {
                    outlines.push(Outline::Audio(audio));
                }
            }
            Event::StartOutlineGroup { text, key } => {
                if let Some(ref mut outlines) = outline_groups.last_mut() {
                    outlines.push(Outline::Group {
                        text: text,
                        key: key,
                        outlines: vec![],
                    });
                }
                outline_groups.push(vec![]);
            }
            Event::EndOutlineGroup => {
                if let Some(ref mut children) = outline_groups.pop() {
                    match outline_groups.last_mut().and_then(|o| o.last_mut()) {
                        Some(&mut Outline::Group { ref mut outlines, .. }) => {
                            outlines.append(children);
                        }
                        Some(_) => {}
                        None => {
                            return Err(Error {
                                description: "End/start elements doesn't match".to_string(),
                            });
                        }
                    }
                }
            }
            _ => {}
        }
    }

    return Ok(document);
}
