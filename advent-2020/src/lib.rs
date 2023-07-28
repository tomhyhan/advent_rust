use std::{fs, error::Error};
use std::time::Instant;


pub trait Runner {
    fn run(&mut self) -> () {
        let start = Instant::now();
        self.part1();
        let duration = start.elapsed();
        println!("part1 ran in : {:?}", duration);

        let start = Instant::now();
        self.part2();
        let duration = start.elapsed();
        println!("part2 ran in : {:?}", duration);
    }

    fn part1(&mut self); 
    fn part2(&mut self); 
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