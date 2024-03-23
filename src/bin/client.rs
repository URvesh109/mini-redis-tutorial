use bytes::Bytes;
use mini_redis::client;
use tokio::sync::mpsc;

#[derive(Debug)]
enum Command {
    Get { key: String },
    Set { key: String, val: Bytes },
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);

    let manager = tokio::spawn(async move {
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();

        while let Some(cmd) = rx.recv().await {
            use Command::*;

            match cmd {
                Get { key } => {
                    let result = client.get(&key).await.unwrap();
                    println!("Get result {:?}", result);
                }
                Set { key, val } => {
                    let result = client.set(&key, val).await.unwrap();
                    println!("Set result {:?}", result);
                }
            }
        }
    });

    let tx2 = tx.clone();

    let t1 = tokio::spawn(async move {
        println!("Send get command");
        let cmd = Command::Get {
            key: "foo".to_string(),
        };
        tx.send(cmd).await.unwrap();
    });

    let t2 = tokio::spawn(async move {
        println!("Send set command");
        let cmd = Command::Set {
            key: "foo".to_string(),
            val: "bar".into(),
        };
        tx2.send(cmd).await.unwrap();
    });

    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();
}
