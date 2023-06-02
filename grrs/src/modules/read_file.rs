use std::fs::read_to_string;
use std::path::PathBuf;

pub fn read(filename: &PathBuf) -> Result<String, anyhow::Error> {
    let content = read_to_string(filename)?;

    return Ok(content);
}
