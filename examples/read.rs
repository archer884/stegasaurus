use std::fs;

fn main() -> stegasaurus::Result<()> {
    let mut content = Vec::new();
    stegasaurus::recover(
        &fs::read("./resource/illuminati-modified.png")?,
        &mut content,
    )?;

    println!("{}", String::from_utf8_lossy(&content));
    Ok(())
}
