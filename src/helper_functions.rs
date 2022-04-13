use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::os::windows::fs::FileExt;

pub fn int2bytes_str(n: i32) -> Vec<u8> {
    n.to_string().as_bytes().to_owned()
}

pub fn read_file(path: &str, offset: u64, length: usize) -> Vec<u8> {
    let mut buf = vec![0u8; length];
    let mut file = File::open(path).unwrap();
    file.seek(SeekFrom::Start(offset)).unwrap();
    file.read_exact(&mut buf).unwrap();
    buf
}

pub fn write_file(path: &str, data: Vec<u8>, offset: u64) {
    let file = File::options().write(true).open(path).unwrap();
    file.seek_write(&data, offset).unwrap();
}