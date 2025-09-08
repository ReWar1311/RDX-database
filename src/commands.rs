use crate::storage::Db;
use crate::protocol;

pub async fn execute(cmd: Vec<String>, db: &Db) -> String {
    match cmd[0].to_uppercase().as_str() {
        "PING" => protocol::encode_simple("PONG"),
        "ECHO" => protocol::encode_bulk(&cmd[1]),
        "SET" => {
            if cmd.len() < 3 {
                return "-ERR wrong number of arguments for 'SET'\r\n".to_string();
            }
            db.set(cmd[1].clone(), cmd[2].clone()).await;
            protocol::encode_simple("OK")
        }
        "GET" => match db.get(&cmd[1]).await {
            Some(v) => protocol::encode_bulk(&v),
            None => protocol::encode_nil(),
        },
        _ => "-ERR unknown command\r\n".to_string(),
    }
}
