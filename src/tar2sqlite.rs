use std::io;
use std::path::Path;

use io::Read;

use rusqlite::Connection;
use rusqlite::Transaction;

use crate::core::TarEntry;
use crate::sqlite::Stmt;
use crate::tar::reader2tar2entries2wtr;

pub fn reader2tar2entries2sqlite<R>(
    rdr: R,
    csql: &str, // create sql
    isql: &str, // insert sql
    con: &mut Connection,
    limit: u64,
) -> Result<(), io::Error>
where
    R: Read,
{
    con.execute(csql, []).map_err(io::Error::other)?;
    let tx: Transaction = con.transaction().map_err(io::Error::other)?;
    {
        let stmt = tx.prepare(isql).map_err(io::Error::other)?;

        let mut statement = Stmt(stmt);
        reader2tar2entries2wtr(
            rdr,
            &mut |ent: &TarEntry| {
                statement.upsert(ent)?;
                Ok(())
            },
            limit,
        )?;
    }
    tx.commit().map_err(io::Error::other)?;
    Ok(())
}

pub fn stdin2tar2entries2sqlite2fs<P>(
    csql: &str, // create sql
    isql: &str, // upsert sql
    dbpath: P,
    limit: u64,
) -> Result<(), io::Error>
where
    P: AsRef<Path>,
{
    let mut con = Connection::open(dbpath).map_err(io::Error::other)?;
    reader2tar2entries2sqlite(io::stdin().lock(), csql, isql, &mut con, limit)
}

pub const CREATE_TABLE_DEFAULT: &str = include_str!("create.sql");
pub const INSERT_TABLE_DEFAULT: &str = include_str!("insert.sql");

pub const BLOB_SIZE_MAX_DEFAULT: u64 = 16777216;

pub fn stdin2tar2entries2sqlite2fs_default<P>(dbpath: P) -> Result<(), io::Error>
where
    P: AsRef<Path>,
{
    stdin2tar2entries2sqlite2fs(
        CREATE_TABLE_DEFAULT,
        INSERT_TABLE_DEFAULT,
        dbpath,
        BLOB_SIZE_MAX_DEFAULT,
    )
}
