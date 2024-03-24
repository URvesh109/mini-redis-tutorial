use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6142").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        // auto
        // tokio::spawn(async move {
        //     let (mut rd, mut wr) = socket.split();
        //     if io::copy(&mut rd, &mut wr).await.is_err() {
        //         eprintln!("failed to copy");
        //     }
        // });

        // manual
        tokio::spawn(async move {
            let mut buf = vec![0; 128];

            loop {
                match socket.read(&mut buf).await {
                    Ok(0) => return,
                    Ok(n) => {
                        if socket.write_all(&buf[..n]).await.is_err() {
                            return;
                        }
                    }
                    Err(_) => {
                        return;
                    }
                }
            }
        });
    }
}
