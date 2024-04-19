use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use anyhow::Result;

pub fn read_file(path: &Path) -> Result<Vec<u8>> {
    let mut f = File::open(path)?;
    let metadata = fs::metadata(path)?;
    let mut buffer = vec![0; metadata.len() as usize];
    f.read_exact(&mut buffer)?;
    Ok(buffer)
}
