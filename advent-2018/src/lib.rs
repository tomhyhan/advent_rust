use std::{fs, error::Error};

pub trait Runner {
    fn run(&mut self);
}


pub fn get_file (file_name: &str) -> Result<String, Box<dyn Error>>{
    let content = fs::read_to_string(file_name)?;
    Ok(content)
}

pub fn copy_file(file_name: &str) -> Result<(), Box<dyn Error>>{
    let content = get_file("src/sample.rs").unwrap();
    let new_content = content.replace("Sample", &file_name.to_uppercase());
    Ok(fs::write(format!("./src/{file_name}.rs"), new_content)?)
}