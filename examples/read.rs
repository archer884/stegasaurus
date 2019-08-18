use std::fs::File;
use std::io::BufReader;
use stegasaurus::Result;

fn main() -> Result<()> {
    let carrier = BufReader::new(File::open("./resource/illuminati-modified.png")?);
    let mut content = Vec::new();

    stegasaurus::recover(carrier, &mut content)?;

    let message: String = content.into_iter().map(|u| u as char).collect();

    println!("{}", message);
    Ok(())
}
