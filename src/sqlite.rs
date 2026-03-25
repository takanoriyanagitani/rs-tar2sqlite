use std::io;

use rusqlite::Statement;

use crate::core::TarEntry;

pub struct Stmt<'a>(pub Statement<'a>);

impl<'a> Stmt<'a> {
    pub fn upsert(&mut self, ent: &TarEntry) -> Result<usize, io::Error> {
        let name: &str = &ent.name;
        let mode: u32 = ent.mode;
        let modified_unixtime: u64 = ent.modified_unixtime;
        let mtime: i64 = modified_unixtime.try_into().map_err(io::Error::other)?;
        let size: u64 = ent.data.len() as u64;
        let sz: i64 = size.try_into().map_err(io::Error::other)?;
        let data: &[u8] = &ent.data;
        self.0
            .execute(rusqlite::params![name, mode, mtime, sz, data,])
            .map_err(io::Error::other)
    }
}
