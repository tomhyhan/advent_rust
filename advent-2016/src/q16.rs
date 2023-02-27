
use crate::Runner;


struct Disk {
    dragon: String,
    length: usize,
}

impl Disk {
    fn new(dragon: String, length:usize) -> Self {
        Disk {
            dragon,
            length
        }
    }
    fn reverse_bits(&self) -> String {
        let mut reverse = "".to_string();
        for char in self.dragon.clone().chars(){
            match char {
                '0' => reverse.push_str("1"),
                '1' => reverse.push_str("0"),
                _ => panic!("not a known bit")
            }
        }
        reverse.chars().rev().collect::<String>()
    }

    fn generate_checksum(&mut self, current_d: String) -> String {
        let current_d = if current_d.len() > self.length {current_d[..self.length].to_string() } else {current_d};
        let checksum = current_d.as_bytes().chunks(2).map(|bs| {
            match bs {
                &[49,49] | &[48,48] => "1",
                _ => "0"
            }
        }).collect::<String>();
        checksum
    }
}

impl Iterator for Disk {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let oppposite = self.reverse_bits();
        let new_dragon = self.dragon.clone() + "0" + &oppposite;
        self.dragon = new_dragon;
        Some(self.dragon.clone())
    }
    
}

pub struct Q16 {}

impl Q16 {
    pub fn new() -> Self {
        Q16 {}
    }

    fn part1(&self) {
        let mut dragon = Disk::new("10001110011110000".to_string(), 35651584);

        while let Some(fill) = dragon.next() {
            if fill.len() >= dragon.length {
                let mut checksum = dragon.generate_checksum(fill);
                while checksum.len() % 2 != 1 {
                    checksum = dragon.generate_checksum(checksum);
                }
                println!("{:?}", checksum);
                break;
            }
        }
    }
}

impl Runner for Q16 {
    fn run(&mut self) -> () {
        self.part1();
    }
}