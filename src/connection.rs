use crate::{app};
use redis::{ErrorKind, RedisError};
use std::net::SocketAddr;
use rusqlite::{Connection};

#[allow(dead_code)]
pub fn get_connection(
    con_info: Vec<redis::ConnectionInfo>,
    cluster_type: &str,
) -> redis::RedisResult<Box<dyn redis::ConnectionLike>> {
    match cluster_type {
        "standlone" => {
            let con = redis::Client::open(con_info[0].clone())?;
            Ok(Box::new(con.get_connection()?))
        }
        "cluster" => {
            let con = redis::cluster::ClusterClient::new(con_info)?;
            
            Ok(Box::new(con.get_connection()?))
        }
        source_type => Err(RedisError::from((
            ErrorKind::InvalidClientConfig,
            "no support source type",
            source_type.to_string(),
        ))),
    }
}


pub fn get_sqlite_con(args: app::Args)-> rusqlite::Result<Connection>{
    let conn = Connection::open(args.db_path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS result (
             id INTEGER PRIMARY KEY,
             key TEXT,
             s_value TEXT,
             t_value TEXT,
             info TEXT
         )",
        [],
    )?;
    Ok(conn)
}




pub fn get_source_conn_info(args: app::Args) -> Vec<redis::ConnectionInfo> {
    let mut db = args.source_db;
    if args.source_db == -1 {
        db = 0
    }
    args.source_address
        .split(';')
        .map(|x| x.to_string().parse().expect("parse address err"))
        .map(|x: SocketAddr| redis::ConnectionInfo {
            addr: redis::ConnectionAddr::Tcp(x.ip().to_string(), x.port()),
            redis: redis::RedisConnectionInfo {
                db,
                username: args.source_user.clone(),
                password: args.source_passwd.clone(),
            },
        })
        .collect()
}

pub fn get_source_conn_info_with_address(args: app::Args,address: String) -> Vec<redis::ConnectionInfo> {
    let mut db = args.source_db;
    if args.source_db == -1 {
        db = 0
    }
    address
        .split(';')
        .map(|x| x.to_string().parse().expect("parse address err"))
        .map(|x: SocketAddr| redis::ConnectionInfo {
            addr: redis::ConnectionAddr::Tcp(x.ip().to_string(), x.port()),
            redis: redis::RedisConnectionInfo {
                db,
                username: args.source_user.clone(),
                password: args.source_passwd.clone(),
            },
        })
        .collect()
}

pub fn get_source_conn_info_with_db(args: app::Args,db: i64) -> Vec<redis::ConnectionInfo> {
    
    args.source_address
        .split(';')
        .map(|x| x.to_string().parse().expect("parse address err"))
        .map(|x: SocketAddr| redis::ConnectionInfo {
            addr: redis::ConnectionAddr::Tcp(x.ip().to_string(), x.port()),
            redis: redis::RedisConnectionInfo {
                db,
                username: args.source_user.clone(),
                password: args.source_passwd.clone(),
            },
        })
        .collect()
}

pub fn get_target_conn_info_with_db(args: app::Args, db: i64) -> Vec<redis::ConnectionInfo> {
    
    args.target_address
        .split(';')
        .map(|x| x.to_string().parse().expect("parse address err"))
        .map(|x: SocketAddr| redis::ConnectionInfo {
            addr: redis::ConnectionAddr::Tcp(x.ip().to_string(), x.port()),
            redis: redis::RedisConnectionInfo {
                db,
                username: args.target_user.clone(),
                password: args.target_passwd.clone(),
            },
        })
        .collect()
}


pub fn get_target_conn_info(args: app::Args) -> Vec<redis::ConnectionInfo> {
    let mut db = args.target_db;
    if args.target_db == -1 {
        db = 0
    }
    args.target_address
        .split(';')
        .map(|x| x.to_string().parse().expect("parse address err"))
        .map(|x: SocketAddr| redis::ConnectionInfo {
            addr: redis::ConnectionAddr::Tcp(x.ip().to_string(), x.port()),
            redis: redis::RedisConnectionInfo {
                db,
                username: args.target_user.clone(),
                password: args.target_passwd.clone(),
            },
        })
        .collect()
}
