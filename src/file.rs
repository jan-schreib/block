use std::path::PathBuf;
use std::io::prelude::*;
use std::fs::File;

#[inline]
fn read_header(file: &PathBuf) -> Option<Vec<u8>>{
    let mut f: File;
    let mut buf = vec![0u8; 4];
    match File::open(file) {
        Ok(i) => f = i,
        Err(e) => panic!(e.to_string()),
    }

    match f.read_exact(&mut buf) {
        Ok(_) => return Some(buf),
        Err(_) => return None,
    }
}

pub fn is_executable(file: &PathBuf) -> bool {
    match read_header(file) {
        Some(h) => {
            match h[0] {
                0x4d => return h[1] == 0x5a,
                0x7f => return h[1] == 0x45 && h[2] == 0x4c && h[3] == 0x46,
                _ => false
            }
        }
        None => false
    }
}
