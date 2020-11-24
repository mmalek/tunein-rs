use std::error::Error;
use std::io::Cursor;
use tokio::stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Fetching main...");

    let client = hyper::Client::new();
    let response = client.get(tunein::request::BROWSE.parse()?).await?;
    let body: Vec<_> = response.into_body().collect::<Result<_, _>>().await?;
    let mut buffer = Vec::new();
    for bytes in body {
        buffer.extend_from_slice(&bytes[..]);
    }

    println!("{:#?}", tunein::read(Cursor::new(buffer))?);

    Ok(())
}
