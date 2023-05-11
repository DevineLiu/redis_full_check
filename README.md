# redis_full_check

Used for comparing the differences between two online Redis servers.

---

## Usage: 
redis_full_check [OPTIONS]
```
Options:
  -s, --source-address <SOURCE_ADDRESS>
          Source address，eg 127.0.0.1:30001;127.0.0.1:30002;127.0.0.1:30003 [default: 127.0.0.1:6379]
      --source-passwd <SOURCE_PASSWD>
          
      --source-user <SOURCE_USER>
          
      --source-db <SOURCE_DB>
          In Redis instances with both ends in standalone mode, databases are iterated based on the number of databases on the source end by default [default: -1]
      --source-type <SOURCE_TYPE>
          Source type,support standalone,cluster [default: cluster]
      --target-type <TARGET_TYPE>
          target type,support standalone,cluster [default: cluster]
      --target-passwd <TARGET_PASSWD>
          
  -t, --target-address <TARGET_ADDRESS>
          Target address [default: 127.0.0.1:6379]
      --target-user <TARGET_USER>
          
      --target-db <TARGET_DB>
          [default: -1]
      --depth <DEPTH>
          This parameter is utilized to specify the depth of comparison for Redis. For a value greater than or equal to 0, only the existence of keys in the source Redis within the target Redis is compared. For a value greater than or equal to 10, the comparison also includes the length of values associated with keys in the source Redis within the target Redis. For a value greater than or equal to 20, the comparison involves the above checks, as well as serialization length and type comparison utilizing the DEBUG OBJECT command. For a value greater than or equal to 30, a comparison of values within the two servers is conducted for equality [default: 10]
      --skip-debug-object <SKIP_DEBUG_OBJECT>
          Whether to skip the comparison using the "debug" command. In the new version of Redis, the "debug" command is disabled by default, and the "debug object" command incurs significant overhead [default: true] [possible values: true, false]
  -d, --db-path <DB_PATH>
          sqlite db file path [default: result.db]
      --batch-size <BATCH_SIZE>
          This parameter determines the batch_size of the results written to SQLite [default: 20]
  -h, --help
          Print help
  -V, --version
          Print version
```