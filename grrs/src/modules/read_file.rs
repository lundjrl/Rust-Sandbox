use std::path::PathBuf;

pub fn read(filename: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(filename)?;

    println!("file content: {}", content);
    Ok(())
}
