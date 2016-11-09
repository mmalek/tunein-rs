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
                document.outlines = outline_groups.pop()
                    .expect("No outline group available")
            }
            Event::Title(title) => document.head.title = title,
            Event::Status(status) => document.head.status = status,
            Event::Link(link) => {
                outline_groups.last_mut()
                    .expect("No outline group available")
                    .push(Outline::Link(link))
            }
            Event::Audio(audio) => {
                outline_groups.last_mut()
                    .expect("No outline group available")
                    .push(Outline::Audio(audio))
            }
            Event::StartOutlineGroup { text, key } => {
                outline_groups.last_mut()
                    .expect("No outline group available")
                    .push(Outline::Group {
                        text: text,
                        key: key,
                        outlines: vec![],
                    });
                outline_groups.push(vec![]);
            }
            Event::EndOutlineGroup => {
                let mut children = outline_groups.pop().expect("No outline group available");
                let mut outline = outline_groups.last_mut()
                    .and_then(|o| o.last_mut())
                    .expect("End/start elements doesn't match");

                match outline {
                    &mut Outline::Group { ref mut outlines, .. } => outlines.append(&mut children),
                    _ => {}
                }
            }
            _ => {}
        }
    }

    return Ok(document);
}
