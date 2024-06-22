use std::fs::OpenOptions;
use std::io::Write;
use reqwest::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = Url::parse("https://www.rust-lang.org/static/images/rust-logo-blk.svg")?;

    let response = reqwest::get(url).await?;
    if response.status().is_success() {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open("rust-logo.svg")?;

        let data = response.bytes().await?;
        file.write_all(&data)?;

        println!("Rust logo downloaded successfully!");
    } else {
        eprintln!("Failed to download the Rust logo. Status: {}", response.status());
    }

    Ok(())
}
