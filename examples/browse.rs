
extern crate hyper;
extern crate tunein;

use std::time::Duration;
use std::error::Error;

fn main()
{
    println!("Fetching main...");

    let mut client = hyper::Client::new();
    client.set_read_timeout(Some(Duration::new(5,0)));
    println!("{:#?}", client.get(tunein::request::BROWSE)
                       .send()
                       .map_err(|e| format!("Connection error: {}", e.description()))
                       .and_then(|response| tunein::read(response)
                                            .map_err(|e| format!("Parse error: {}", e.description()))));
}
