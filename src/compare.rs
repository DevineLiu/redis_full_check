use lazy_static::lazy_static;
use log::{debug, warn};
use redis::{self, ConnectionLike};
use regex::{Regex};
const TARGET_KEY_MISS: &str = "target key missing";

lazy_static! {
    static ref RE: Regex = Regex::new(r"encoding:(\S+)\s+serializedlength:(\d+)").unwrap();
}

pub struct Comparator {
    source_con: Box<dyn ConnectionLike>,
    source_check_con: Box<dyn ConnectionLike>,
    target_con: Box<dyn ConnectionLike>,
    depth: i32,
    skip_debug_object: bool,
    sql_con: rusqlite::Connection,
    batch_size: usize
}

#[derive(Default, Debug,Clone)]
pub struct ResultMessage {
    key: String,
    s_value: Option<redis::Value>,
    t_value: Option<redis::Value>,
    info: Option<String>,
}

pub fn batch_insert_message(conn: &rusqlite::Connection, messages: &Vec<ResultMessage>) -> rusqlite::Result<()> {
    let mut stmt = conn.prepare(
        "INSERT INTO result (key, s_value, t_value, info)
         VALUES (?1, ?2, ?3, ?4)",
    )?;
    for result_message in messages {
        stmt.execute([
            &result_message.key,
            &result_message.s_value.clone().map(|v| {let r: String = redis::FromRedisValue::from_redis_value(&v).unwrap_or_default();r}).unwrap_or_default(),
            &result_message.t_value.clone().map(|v| {let r: String = redis::FromRedisValue::from_redis_value(&v).unwrap_or_default();r}).unwrap_or_default(),
            &result_message.info.clone().unwrap_or_default(),
        ])?;
    };
    Ok(())
}


impl Comparator {
    pub fn new(
        source_con: Box<dyn ConnectionLike>,
        source_check_con: Box<dyn ConnectionLike>,
        target_con: Box<dyn ConnectionLike>,
        sql_con: rusqlite::Connection,
        depth: i32,
        batch_size: usize,
        skip_debug_object: Option<bool>,

    ) -> Self {
        let skip_debug = match skip_debug_object {
            Some(x)=> {x}
            None =>{true}
        };
        Self {
            source_con,
            source_check_con,
            target_con,
            depth,
            skip_debug_object: skip_debug,
            sql_con,
            batch_size,
        }
    }

    pub fn compare(&mut self) -> redis::RedisResult<()> {
        let cursor: u64 = 0;
        let iter: redis::Iter<redis::Value> = redis::cmd("scan")
            .cursor_arg(cursor)
            .clone()
            .iter(self.source_con.as_mut())?;
        let mut diff_message = vec![];
        
        for source_key in iter{
            let  length= diff_message.len();
            if length > self.batch_size {
                batch_insert_message(&self.sql_con, &diff_message).expect("insert_err");
                diff_message.clear();
            }
            let key: String = redis::from_redis_value(&source_key)?;

            if self.depth >= 0 {
                let target_value: redis::Value = self
                    .target_con
                    .req_command(redis::cmd("EXISTS").arg(&key))?;
                match target_value {
                    redis::Value::Int(r) => {
                        if r == 0 {
                            diff_message.push(ResultMessage {
                                key,
                                info: Some(TARGET_KEY_MISS.to_string()),
                                ..Default::default()
                            });
                            continue;
                        }
                    }
                    _ => {
                        warn!("exists ERR")
                    }
                }
            }
            if self.depth >= 10 {
                let target_type: redis::Value =
                    self.target_con.req_command(redis::cmd("TYPE").arg(&key))?;
                let source_type: redis::Value = self.source_check_con.req_command(redis::cmd("TYPE").arg(&key))?;
                if target_type != source_type {
                    diff_message.push(ResultMessage {
                        key,
                        info: Some(format!("type diff")),
                        ..Default::default()
                    });
                    continue;
                }
                match target_type {
                    redis::Value::Status(_type) => match _type.as_str() {
                        "string" => {
                            let target_len: redis::Value = self
                                .target_con
                                .req_command(redis::cmd("STRLEN").arg(&key))?;
                            let source_len: redis::Value = self
                                .source_check_con
                                .req_command(redis::cmd("STRLEN").arg(&key))?;
                            debug!(" key: {:?} len t:{:?} s:{:?}", key, target_len, source_len);
                            if target_len != source_len {
                                let t_len: i64 =
                                    redis::FromRedisValue::from_redis_value(&target_len)?;
                                let s_len: i64 =
                                    redis::FromRedisValue::from_redis_value(&source_len)?;
                                diff_message.push(ResultMessage {
                                    key,
                                    info: Some(format!("len diff t:{} s:t{}", t_len, s_len)),
                                    ..Default::default()
                                });
                                continue;
                            }
                        }
                        "hash" => {
                            let target_len: redis::Value =
                                self.target_con.req_command(redis::cmd("HLEN").arg(&key))?;
                            let source_len: redis::Value = self
                                .source_check_con
                                .req_command(redis::cmd("HLEN").arg(&key))?;
                            debug!(" key: {:?} hlen t:{:?} s:{:?}", key, target_len, source_len);
                            if target_len != source_len {
                                let t_len: i64 =
                                    redis::FromRedisValue::from_redis_value(&target_len)?;
                                let s_len: i64 =
                                    redis::FromRedisValue::from_redis_value(&source_len)?;
                                diff_message.push(ResultMessage {
                                    key,
                                    info: Some(format!("hash len diff t:{} s:t{}", t_len, s_len)),
                                    ..Default::default()
                                });
                                continue;
                            }
                        }
                        "list" => {
                            let target_len: redis::Value =
                                self.target_con.req_command(redis::cmd("LLEN").arg(&key))?;
                            let source_len: redis::Value = self
                                .source_check_con
                                .req_command(redis::cmd("LLEN").arg(&key))?;
                            debug!(" key: {:?} llen t:{:?} s:{:?}", key, target_len, source_len);
                            if target_len != source_len {
                                let t_len: i64 =
                                    redis::FromRedisValue::from_redis_value(&target_len)?;
                                let s_len: i64 =
                                    redis::FromRedisValue::from_redis_value(&source_len)?;
                                diff_message.push(ResultMessage {
                                    key,
                                    info: Some(format!("list len diff t:{} s:t{}", t_len, s_len)),
                                    ..Default::default()
                                });
                                continue;
                            }
                        }
                        "stream" => {
                            let target_len: redis::Value =
                                self.target_con.req_command(redis::cmd("XLEN").arg(&key))?;
                            let source_len: redis::Value = self
                                .source_check_con
                                .req_command(redis::cmd("XLEN").arg(&key))?;
                            debug!(" key: {:?} xlen t:{:?} s:{:?}", key, target_len, source_len);
                            if target_len != source_len {
                                let t_len: i64 =
                                    redis::FromRedisValue::from_redis_value(&target_len)?;
                                let s_len: i64 =
                                    redis::FromRedisValue::from_redis_value(&source_len)?;
                                diff_message.push(ResultMessage {
                                    key,
                                    info: Some(format!("stream len diff t:{} s:t{}", t_len, s_len)),
                                    ..Default::default()
                                });
                                continue;
                            }
                        }
                        "set" => {
                            let target_len: redis::Value =
                                self.target_con.req_command(redis::cmd("SCARD").arg(&key))?;
                            let source_len: redis::Value = self
                                .source_check_con
                                .req_command(redis::cmd("SCARD").arg(&key))?;
                            debug!(
                                " key: {:?} SCARD t:{:?} s:{:?}",
                                key, target_len, source_len
                            );
                            if target_len != source_len {
                                let t_len: i64 =
                                    redis::FromRedisValue::from_redis_value(&target_len)?;
                                let s_len: i64 =
                                    redis::FromRedisValue::from_redis_value(&source_len)?;
                                diff_message.push(ResultMessage {
                                    key,
                                    info: Some(format!("set len diff t:{} s:t{}", t_len, s_len)),
                                    ..Default::default()
                                });
                                continue;
                            }
                        }
                        "zset" => {
                            let target_len: redis::Value =
                                self.target_con.req_command(redis::cmd("ZCARD").arg(&key))?;
                            let source_len: redis::Value = self
                                .source_check_con
                                .req_command(redis::cmd("ZCARD").arg(&key))?;
                            debug!(
                                " key: {:?} ZCARD t:{:?} s:{:?}",
                                key, target_len, source_len
                            );
                            if target_len != source_len {
                                let t_len: i64 =
                                    redis::FromRedisValue::from_redis_value(&target_len)?;
                                let s_len: i64 =
                                    redis::FromRedisValue::from_redis_value(&source_len)?;
                                diff_message.push(ResultMessage {
                                    key,
                                    info: Some(format!("zset len diff t:{} s:t{}", t_len, s_len)),
                                    ..Default::default()
                                });
                                continue;
                            }
                        }
                        x => {
                            warn!("unknown type {}", x)
                        }
                    },
                    _ => {}
                }
            }

            if self.depth >= 20 && !self.skip_debug_object {
                let object_info: redis::Value = self
                    .target_con
                    .req_command(redis::cmd("DEBUG").arg("OBJECT").arg(&key))?;
                let value: String = redis::FromRedisValue::from_redis_value(&object_info)?;
                let (t_encoding, t_length) = match extract_encoding_and_length(value) {
                    Some((encoding, length)) => (encoding, length),
                    None => ("".to_string(), -1),
                };
                let object_info: redis::Value = self
                    .source_check_con
                    .req_command(redis::cmd("DEBUG").arg("OBJECT").arg(&key))?;
                let value: String = redis::FromRedisValue::from_redis_value(&object_info)?;
                let (s_encoding, s_length) = match extract_encoding_and_length(value) {
                    Some((encoding, length)) => (encoding, length),
                    None => ("".to_string(), -1),
                };
                debug!(
                    "debug object: {}  t_encoding:{} s_encoding:{} t_length:{} s_length:{}",
                    key, t_encoding, s_encoding, t_length, s_length
                );
                if t_encoding != s_encoding || t_length != s_length {
                    diff_message.push(ResultMessage {
                        key,
                        info: Some("debug diff".to_string()),
                        ..Default::default()
                    });
                    continue;
                }
            }

            if self.depth >= 30 {
                let target_type: redis::Value =
                    self.target_con.req_command(redis::cmd("TYPE").arg(&key))?;
                match target_type {
                    redis::Value::Status(_type) => match _type.as_str() {
                        "string" => {
                            let target_value: redis::Value =
                                self.target_con.req_command(redis::cmd("GET").arg(&key))?;
                            let source_value: redis::Value = self
                                .source_check_con
                                .req_command(redis::cmd("GET").arg(&key))?;
                            if target_value != source_value {
                                diff_message.push(ResultMessage {
                                    key,
                                    info: Some("string diff value".to_string()),
                                    ..Default::default()
                                });
                                continue;
                            }
                        }
                        "hash" => {
                            let target_value: redis::Value =
                                self.target_con.req_command(redis::cmd("HVALS").arg(&key))?;
                            let source_value: redis::Value = self
                                .source_check_con
                                .req_command(redis::cmd("HVALS").arg(&key))?;

                            if target_value != source_value {
                                diff_message.push(ResultMessage {
                                    key,
                                    info: Some("hash diff value".to_string()),
                                    ..Default::default()
                                });
                                continue;
                            }
                        }
                        "list" => {
                            let source_len: redis::Value = self
                                .source_check_con
                                .req_command(redis::cmd("LLEN").arg(&key))?;
                            let s_len: i64 = redis::FromRedisValue::from_redis_value(&source_len)?;
                            let source_value: redis::Value = self
                                .source_check_con
                                .req_command(redis::cmd("LRANGE").arg(&key).arg(0).arg(s_len))?;
                            let target_value: redis::Value = self
                                .target_con
                                .req_command(redis::cmd("LRANGE").arg(&key).arg(0).arg(s_len))?;
                            if source_value != target_value {
                                diff_message.push(ResultMessage {
                                    key,
                                    info: Some("list diff value".to_string()),
                                    ..Default::default()
                                });
                                continue;
                            }
                        }
                        "set" => {
                            let source_value: redis::Value = self
                                .source_check_con
                                .req_command(redis::cmd("SMEMBERS").arg(&key))?;
                            let target_value: redis::Value = self
                                .target_con
                                .req_command(redis::cmd("SMEMBERS").arg(&key))?;
                            if target_value != source_value {
                                diff_message.push(ResultMessage {
                                    key,
                                    info: Some("set diff value".to_string()),
                                    ..Default::default()
                                });
                                continue;
                            }
                        }
                        "zset" => {
                            let source_len: redis::Value = self
                                .source_check_con
                                .req_command(redis::cmd("ZCARD").arg(&key))?;
                            let s_len: i64 = redis::FromRedisValue::from_redis_value(&source_len)?;
                            let source_value: redis::Value = self
                                .source_check_con
                                .req_command(redis::cmd("ZRANGE").arg(&key).arg(0).arg(s_len))?;
                            let target_value: redis::Value = self
                                .target_con
                                .req_command(redis::cmd("ZRANGE").arg(&key).arg(0).arg(s_len))?;
                            if source_value != target_value {
                                diff_message.push(ResultMessage {
                                    key,
                                    info: Some("zset diff value".to_string()),
                                    ..Default::default()
                                });
                                continue;
                            }
                        }

                        x => {
                            warn!("unknown type {}", x)
                        }
                    },
                    t => {
                        warn!("unkown type {:?} key:  {}", t, key)
                    }
                }
            }
        }
        batch_insert_message(&self.sql_con, &diff_message).expect("insert_err");
       
        Ok(())
    }
}

fn extract_encoding_and_length(s: String) -> Option<(String, i64)> {
    
    if let Some(caps) = RE.captures(s.as_str()) {
        let encoding = caps[1].to_string();
        let length = caps[2].parse().expect("no serialized length");
        Some((encoding, length))
    } else {
        None
    }
}
