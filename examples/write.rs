use std::fs::File;
use std::io::BufReader;
use stegasaurus::{Message, Result};

fn main() -> Result<()> {
    let message = Message::new(b"Hello, world!");
    let carrier = BufReader::new(File::open("./resource/illuminati.png")?);
    let target = File::create("./resource/illuminati-modified.png")?;

    message.store(carrier, target)
}
