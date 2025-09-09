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
        }
        "DEL" => {
            if cmd.len() < 2 {
                return "-ERR wrong number of arguments for 'DEL'\r\n".to_string();
            }
            db.del(&cmd[1]).await;
            protocol::encode_simple("OK")
        }
        "EXISTS" => {
            if cmd.len() < 2 {
                return "-ERR wrong number of arguments for 'EXISTS'\r\n".to_string();
            }
            if db.exists(&cmd[1]).await {
                protocol::encode_simple("1")
            } else {
                protocol::encode_simple("0")
            }
        }
        "MSET" => {
            if cmd.len() < 3 {
                return "-ERR wrong number of arguments for 'MSET'\r\n".to_string();
            }
            let mut i = 1;
            while i < cmd.len() {
                db.set(cmd[i].clone(), cmd[i + 1].clone()).await;
                i += 2;
            }
            protocol::encode_simple("OK")
        }
        "MGET" => {
            if cmd.len() < 2 {
                return "-ERR wrong number of arguments for 'MGET'\r\n".to_string();
            }
            let values = db.mget(&cmd[1..]).await;
            protocol::encode_array(values)
        }
        "APPEND" => {
            if cmd.len() < 3 {
                return "-ERR wrong number of arguments for 'APPEND'\r\n".to_string();
            }
            db.append(&cmd[1], cmd[2].clone()).await;
            protocol::encode_simple("OK")
        }
        "RENAME" => {
            if cmd.len() < 3 {
                return "-ERR wrong number of arguments for 'RENAME'\r\n".to_string();
            }
            if db.rename(&cmd[1], &cmd[2]).await {
                protocol::encode_simple("OK")
            } else {
                protocol::encode_simple("0")
            }
        }
        _ => "-ERR unknown command\r\n".to_string(),
    }
}
