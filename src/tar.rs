use std::borrow::Cow;
use std::io;
use std::path::Path;

use io::Read;

use tar::Archive;
use tar::Entry;
use tar::Header;

use crate::core::TarEntry;

pub fn header2size(hdr: &Header) -> Result<u64, io::Error> {
    hdr.size()
}

pub fn header2mode(hdr: &Header) -> Result<u32, io::Error> {
    hdr.mode()
}

pub fn header2mtime(hdr: &Header) -> Result<u64, io::Error> {
    hdr.mtime()
}

pub fn ent2path<'a, R>(ent: &'a Entry<R>) -> Result<Cow<'a, Path>, io::Error>
where
    R: Read,
{
    ent.path()
}

pub fn ent2hdr<'a, R>(ent: &'a Entry<R>) -> &'a Header
where
    R: Read,
{
    ent.header()
}

pub fn ent2data<R>(ent: &mut Entry<R>, buf: &mut Vec<u8>, limit: u64) -> Result<usize, io::Error>
where
    R: Read,
{
    buf.clear();
    let mut taken = ent.take(limit);
    taken.read_to_end(buf)
}

pub fn copy<R>(src: &mut Entry<R>, dst: &mut TarEntry, limit: u64) -> Result<usize, io::Error>
where
    R: Read,
{
    let hdr: &Header = ent2hdr(src);

    let mode: u32 = header2mode(hdr)?;
    let unixtime: u64 = header2mtime(hdr)?;

    let pat: &Path = &ent2path(src)?;
    let ostr: Option<&str> = pat.to_str();
    let upat: &str = ostr.ok_or(io::Error::other("invalid path"))?;
    dst.name.clear();
    dst.name.push_str(upat);

    let usz: usize = ent2data(src, &mut dst.data, limit)?;

    dst.mode = mode;
    dst.modified_unixtime = unixtime;
    dst.size = usz as u64;

    Ok(usz)
}

pub fn reader2tar2entries2wtr<R, W>(rdr: R, wtr: &mut W, limit: u64) -> Result<(), io::Error>
where
    R: Read,
    W: FnMut(&TarEntry) -> Result<(), io::Error>,
{
    let mut ta = Archive::new(rdr);
    let entries = ta.entries()?;
    let mut buf = TarEntry::default();
    for rent in entries {
        let mut ent = rent?;
        copy(&mut ent, &mut buf, limit)?;
        wtr(&buf)?;
    }
    Ok(())
}
