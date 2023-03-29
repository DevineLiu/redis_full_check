use clap::{self, Parser};

#[derive(Parser, Debug, Clone)]
#[clap(author="hrliu", version="v0.1", about="redis check helper", long_about = None)]
pub struct Args {
    /// Source address，eg 127.0.0.1:30001;127.0.0.1:30002;127.0.0.1:30003
    #[clap(short, long, value_parser, default_value = "127.0.0.1:6379")]
    pub source_address: String,

    #[clap(long, value_parser)]
    pub source_passwd: Option<String>,

    #[clap(long, value_parser)]
    pub source_user: Option<String>,

    /// In Redis instances with both ends in standalone mode, databases are iterated based on the number of databases on the source end by default.
    #[clap(long, value_parser, default_value = "-1")]
    pub source_db: i64,

    /// Source type,support standlone,cluster
    #[clap(long, value_parser, default_value = "cluster")]
    pub source_type: String,

    /// target type,support standlone,cluster
    #[clap(long, value_parser, default_value = "cluster")]
    pub target_type: String,

    // target password
    #[clap(long, value_parser)]
    pub target_passwd: Option<String>,
    /// Target address
    #[clap(short, long, value_parser, default_value = "127.0.0.1:6379")]
    pub target_address: String,
    ///
    #[clap(long, value_parser)]
    pub target_user: Option<String>,

    #[clap(long, value_parser, default_value = "-1")]
    pub target_db: i64,
    /// This parameter is utilized to specify the depth of comparison for Redis. 
    /// For a value greater than or equal to 0, only the existence of keys in the source Redis within the target Redis is compared.
    /// For a value greater than or equal to 10, the comparison also includes the length of values associated with keys in the source Redis within the target Redis.
    /// For a value greater than or equal to 20, the comparison involves the above checks, as well as serialization length and type comparison utilizing the DEBUG OBJECT command. 
    /// For a value greater than or equal to 30, a comparison of values within the two servers is conducted for equality.
    #[clap(long, value_parser, default_value = "10")]
    pub depth: i32,

    /// Whether to skip the comparison using the "debug" command. In the new version of Redis, the "debug" command is disabled by default, and the "debug object" command incurs significant overhead.
    #[clap(long, value_parser, default_value = "true")]
    pub skip_debug_object: Option<bool>,

    /// sqlite db file path
    #[clap(short, long, value_parser, default_value = "result.db")]
    pub db_path: String,
    /// This parameter determines the batch_size of the results written to SQLite.
    #[clap(long, value_parser, default_value = "20")]
    pub batch_size: usize,
}
