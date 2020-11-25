use crate::common::{Document, Group, Outline};
use crate::error::Error;
use crate::event::Event;
use crate::reader::Reader;

use std::io::Read;

pub fn read<R: Read>(source: R) -> Result<Document, Error> {
    let mut document = Document::default();

    let mut outline_stack: Vec<Outline> = vec![];

    for event in Reader::new(source) {
        match event? {
            Event::StartDocument { version } => {
                document.version.major = version;
            }
            Event::EndDocument => {
                if !outline_stack.is_empty() {
                    unreachable!("Outline stack is not empty");
                }
            }
            Event::Title(title) => document.head.title = title,
            Event::Status(status) => document.head.status = status,
            Event::StartOutline(outline) => outline_stack.push((*outline).into()),
            Event::EndOutline => {
                let outline = outline_stack
                    .pop()
                    .expect("End/start elements doesn't match");

                let outlines = outline_stack
                    .last_mut()
                    .map(|o| match *o {
                        Outline::Group(Group {
                            ref mut outlines, ..
                        }) => outlines,
                        _ => unreachable!("Last outline is not group"),
                    })
                    .unwrap_or(&mut document.outlines);

                outlines.push(outline);
            }
            _ => {}
        }
    }

    return Ok(document);
}
