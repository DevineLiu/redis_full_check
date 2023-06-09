use anyhow::{Ok, Result};
use clap::Parser;
use log::info;
use redis::{self};
use redis_full_check::app::Args;
use redis_full_check::compare::Comparator;
use redis_full_check::connection;

fn main() -> Result<()> {
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );

    let _args = Args::parse();

    if _args.source_type == _args.target_type
        && _args.source_db == _args.target_db
        && _args.source_type == "standalone"
        && _args.source_db == -1
    {
        
        let source_conns_info = connection::get_source_conn_info(_args.clone());
        let source_con = connection::get_connection(source_conns_info, &_args.source_type);

        let source_value: redis::Value = source_con
            .expect("")
            .as_mut()
            .req_command(redis::cmd("CONFIG").arg("GET").arg("DATABASES"))?;
        let db_info: Vec<String> = redis::FromRedisValue::from_redis_value(&source_value)?;
        let db_num: i64 = db_info[1].parse()?;
        info!("Compare all the databases of the source.");
        for db in 0..db_num {
            info!("Comparing db:{}",db);
            let sql_con = connection::get_sqlite_con(_args.clone());
            let source_conns_info = connection::get_source_conn_info_with_db(_args.clone(), db);
            let source_con =
                connection::get_connection(source_conns_info.clone(), &_args.source_type);
            let source_check_con =
                connection::get_connection(source_conns_info.clone(), &_args.source_type);
            let conns_info = connection::get_target_conn_info_with_db(_args.clone(), db);
            let target_con = connection::get_connection(conns_info, &_args.target_type);
            let mut comparator = Comparator::new(
                source_con?,
                source_check_con?,
                target_con?,
                sql_con?,
                _args.depth,
                _args.batch_size,
                _args.skip_debug_object,
            );
            comparator.compare()?;
        }
    } else if _args.source_type == "cluster" {

        for addr in _args.source_address.split(';') {
            info!("Compare the shard you provided. {}",addr);
            let source_con_standalone =
                connection::get_source_conn_info_with_address(_args.clone(), addr.to_string());
            let source_con = connection::get_connection(source_con_standalone.clone(), "standalone");

            let source_conns_info = connection::get_source_conn_info(_args.clone());
            let source_check_con =
                connection::get_connection(source_conns_info, &_args.source_type);
            let conns_info = connection::get_target_conn_info(_args.clone());
            let target_con = connection::get_connection(conns_info, &_args.target_type);
            let sql_con = connection::get_sqlite_con(_args.clone());
            let mut comparator = Comparator::new(
                source_con?,
                source_check_con?,
                target_con?,
                sql_con?,
                _args.depth,
                _args.batch_size,
                _args.skip_debug_object,
            );
            
            comparator.compare()?;
            
        }
    } else {
        let source_conns_info = connection::get_source_conn_info(_args.clone());
        let source_con = connection::get_connection(source_conns_info.clone(), &_args.source_type);
        let source_check_con = connection::get_connection(source_conns_info, &_args.source_type);
        let conns_info = connection::get_target_conn_info(_args.clone());
        let target_con = connection::get_connection(conns_info, &_args.target_type);
        let sql_con = connection::get_sqlite_con(_args.clone());
        let mut comparator = Comparator::new(
            source_con?,
            source_check_con?,
            target_con?,
            sql_con?,
            _args.depth,
            _args.batch_size,
            _args.skip_debug_object,
        );
        comparator.compare()?;
    };
    info!(
        "success!!! result in sqlite3: {}   `result` table",
        _args.db_path
    );
    Ok(())
}
