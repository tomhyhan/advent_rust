use std::fs;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)] 
struct Dimension {
    length: i32,
    width: i32,
    height: i32,
}

impl Dimension {
    fn surface(&self) -> i32 {
        &self.length * &self.width * 2 + &self.width * &self.height * 2 + &self.height * &self.length * 2
    }
    fn find_smallest(&self) -> i32 {
        *vec![&self.length * &self.width, &self.width * &self.height, &self.height * &self.length].iter().min().unwrap()
    }
    //  Vec<Option<i32>>
    fn find_two_smallest(&mut self) {

    }
}

fn turn_to_dimension(area_string: &str) ->Dimension {
    println!("{area_string}");
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\d+").unwrap();
    }
    let x : Vec<i32>= RE.find_iter(area_string).filter_map(|d| d.as_str().parse().ok()).collect();
    println!("{:?}",x);
    Dimension{
        length : x[0],
        width : x[1], 
        height : x[2],
    }
}
//  2*6 + 2*12 + 2*8 = 52 + 6(smallest)
fn main() {
    let test = Dimension {width: 1, height: 2, length:3};
    println!("{:?}", test.find_two_smallest());
    let path = "input.txt";
    let context = fs::read_to_string(path).expect("asdf");
    // println!("{:?}", context);
    let areas: Vec<&str> = context.split('\n').collect();
    // println!("{:?}", areas);
    let dimensions: Vec<_>= areas.iter().map(|area_string| turn_to_dimension(area_string)).collect();
    println!("{:?}",dimensions);
    let result: i32 = dimensions.iter().fold(0, |acc,d| {
        println!("{:?}", d.find_two_smallest());
        acc +  d.surface() + d.find_smallest() 
    });
    println!("{result}");
}
