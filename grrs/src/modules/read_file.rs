use std::path::PathBuf;

pub fn read(filename: &PathBuf) -> Result<String, anyhow::Error> {
    let content = std::fs::read_to_string(filename)?;

    println!("file content: {}", content);

    return Ok(content);
}
