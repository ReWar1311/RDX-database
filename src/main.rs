mod server;
mod commands;
mod protocol;
mod storage;

use storage::Db;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db = Db::new();
    server::run("127.0.0.1:6379", db).await?;
    Ok(())
}
