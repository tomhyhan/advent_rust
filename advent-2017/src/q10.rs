use advent_2017::{Runner, get_file};

pub struct Q10 {
    input: Vec<usize>,
}

struct Knot {
    list: Vec<usize>,
    length: usize,
    skip: usize,
    current_pos: usize
}

impl Knot {
    fn new(elements:usize) -> Self {
        let list: Vec<usize> = (0..elements).into_iter().collect();
        let length = list.len();
        let skip = 0;
        let current_pos = 0;
        Knot {list,length,skip,current_pos}
    }
    
    fn hash(&mut self, des: usize) {
        //  0 1 2  
        for d  in 0..(des/2) {
            let left = (d + self.current_pos) % self.length;
            let right = (des + self.current_pos - d - 1) % self.length;
            self.list.swap(left, right);
        }

        self.current_pos = (self.current_pos + des + self.skip) % self.length;
        self.skip += 1;
    }
    


}

impl Q10 {
    pub fn new() -> Self {
        let content = get_file("q10.txt").unwrap();
        // part 1
        // let input: Vec<usize> = content.split(",").map(|num|num.parse::<usize>().unwrap()).collect();

        let mut input: Vec<usize> = content.chars().map(|c|( c as u8 ) as usize).collect();
        let mut x: Vec<usize> = vec![17,31,73,47,23];
        input.append(&mut x);
        Q10 {input}
    }

    fn part1(&mut self){
        let mut knot = Knot::new(256);
        for des in self.input.iter() {
            knot.hash(*des);
        }
        println!("{:?}", knot.list);
        println!("{:?}", knot.list[0] * knot.list[1]);
    }

    fn part2(&mut self) {

        let mut knot = Knot::new(256);
        for _ in 0..64 {
            for des in self.input.iter() {
                knot.hash(*des);
            }
        }

        let mut answer = "".to_string();
        for c in knot.list.chunks(16) {
            answer.push_str(&format!("{:02x}", c.iter().fold(0, |x,y|x ^ *y)));
        }
        println!("{answer}")
    }
}

impl Runner for Q10 {
    fn run(&mut self) {
        self.part1();
        self.part2();
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn reverse_list() {

    }    
}