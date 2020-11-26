use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use std::error::Error;
use std::io::Cursor;
use tokio::stream::StreamExt;
use tunein::Outline;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let query = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "Kraków".to_string());

    let query = utf8_percent_encode(&query, NON_ALPHANUMERIC);

    let uri = tunein::request::search_uri(&query);

    let client = hyper::Client::new();
    let response = client.get(uri.parse()?).await?;
    let mut body = response.into_body();

    let mut buffer = Vec::new();
    while let Some(bytes) = body.next().await {
        buffer.extend_from_slice(&bytes?);
    }

    let document = tunein::read(Cursor::new(buffer))?;

    println!("{}\n-----", document.head.title);

    print_outlines(&document.outlines, 0);

    Ok(())
}

fn print_outlines(outlines: &[Outline], depth: usize) {
    for outline in outlines {
        print!("{:>indent$}", "", indent = 4 * depth);
        match outline {
            Outline::Group(group) => {
                println!("{}:", group.text);
                print_outlines(&group.outlines, depth + 1);
            }
            Outline::Link(link) => println!("{} - {}", link.text, link.url),
            Outline::Audio(audio) => println!("{} - {}", audio.text, audio.url),
            Outline::Text(text) => println!("{}", text),
        }
    }
}
