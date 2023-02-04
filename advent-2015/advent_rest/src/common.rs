use std::error::Error;
use std::fs;

pub fn read_file(path: &str) -> Result<String, Box<dyn Error>> {
    let content = fs::read_to_string(path)?;
    Ok(content)
}
