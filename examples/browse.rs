use std::error::Error;
use std::io::Cursor;
use tokio::stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Browsing...");

    let client = hyper::Client::new();
    let response = client.get(tunein::request::BROWSE_URI.parse()?).await?;
    let mut body = response.into_body();

    let mut buffer = Vec::new();
    while let Some(bytes) = body.next().await {
        buffer.extend_from_slice(&bytes?);
    }

    println!("{:#?}", tunein::read(Cursor::new(buffer))?);

    Ok(())
}
