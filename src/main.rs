use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut f = File::open("foo.txt").await?;

    let mut buff = Vec::new();

    let n = f.read_to_end(&mut buff).await?;

    let file_text = String::from_utf8_lossy(&buff[..n]).to_string();

    println!("{file_text}");

    Ok(())
}
