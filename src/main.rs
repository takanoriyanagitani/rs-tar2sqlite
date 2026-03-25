use std::fs;
use std::io;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;

use rs_tar2sqlite::tar2sqlite::BLOB_SIZE_MAX_DEFAULT;
use rs_tar2sqlite::tar2sqlite::CREATE_TABLE_DEFAULT;
use rs_tar2sqlite::tar2sqlite::INSERT_TABLE_DEFAULT;
use rs_tar2sqlite::tar2sqlite::stdin2tar2entries2sqlite2fs;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to the SQLite database
    db_path: PathBuf,

    #[arg(long, default_value = "")]
    create_sql: String,

    #[arg(long, default_value = "")]
    insert_sql: String,

    #[arg(long, default_value_t = BLOB_SIZE_MAX_DEFAULT)]
    max_blob_size: u64,
}

fn sub() -> Result<(), io::Error> {
    let args = Cli::parse();

    let csql = if args.create_sql.is_empty() {
        CREATE_TABLE_DEFAULT.to_string()
    } else {
        fs::read_to_string(args.create_sql)?
    };

    let isql = if args.insert_sql.is_empty() {
        INSERT_TABLE_DEFAULT.to_string()
    } else {
        fs::read_to_string(args.insert_sql)?
    };

    stdin2tar2entries2sqlite2fs(&csql, &isql, args.db_path, args.max_blob_size)?;
    Ok(())
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        ExitCode::FAILURE
    })
}
