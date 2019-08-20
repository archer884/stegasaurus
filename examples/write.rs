use std::fs::{self, File};

fn main() -> stegasaurus::Result<()> {
    stegasaurus::store(
        b"Hello, world!",
        &fs::read("./resource/illuminati.png")?,
        File::create("./resource/illuminati-modified.png")?,
    )
}
