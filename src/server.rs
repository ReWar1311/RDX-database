use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::{protocol, commands};
use crate::storage::Db;

pub async fn run(addr: &str, db: Db) -> anyhow::Result<()> {
    let listener = TcpListener::bind(addr).await?;
    println!("RDX server listening on {}", addr);

    loop {
        let (mut socket, _) = listener.accept().await?;
        let db = db.clone();

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];

            loop {
                match socket.read(&mut buf).await {
                    Ok(0) => return, // client disconnected
                    Ok(n) => {
                        let input = &buf[..n];
                        if let Ok(cmd) = protocol::parse(input) {
                            let resp = commands::execute(cmd, &db).await;
                            let _ = socket.write_all(resp.as_bytes()).await;
                        } else {
                            let _ = socket.write_all(b"-ERR invalid command\r\n").await;
                        }
                    }
                    Err(_) => return,
                }
            }
        });
    }
}
