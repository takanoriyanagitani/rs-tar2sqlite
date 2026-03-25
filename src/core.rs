#[derive(Default)]
pub struct TarEntry {
    pub name: String,
    pub mode: u32,
    pub modified_unixtime: u64,
    pub size: u64,
    pub data: Vec<u8>,
}
