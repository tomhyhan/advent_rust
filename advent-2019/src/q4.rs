use std::collections::HashMap;

use advent_2019::Runner;

pub struct Q4 {

}

struct Container {
    password: i32
}

impl Container {
    fn new(password:i32) -> Self {
        Self {password}
    }

    fn check_password(&mut self) -> bool {
        let passwords_bytes = format!("{:?}", self.password).as_bytes().to_owned();        
        let mut sorted = passwords_bytes.clone();
        sorted.sort();
        let is_double = passwords_bytes.iter().fold(HashMap::new(), |mut acc, &num| { *acc.entry(num).or_insert(0) += 1; acc }).into_iter().find(|(_,v)| *v == 2).is_some();

        sorted == passwords_bytes && is_double
    } 
    // fn check_password(&self) -> bool {
    //     let mut is_double = false;
    //     let passwords_bytes = format!("{:?}", self.password).as_bytes().to_owned();
    //     for (idx, bytes) in passwords_bytes.windows(2).enumerate() {
    //         let (num1, num2) = ((bytes[0] as char).to_digit(10).unwrap(), (bytes[1] as char).to_digit(10).unwrap());
    //         if !is_double && num1 == num2 {
    //             let mut is_double_sub = true;
    //             match idx { 
    //                 0 => {
    //                     if passwords_bytes[idx+2] == passwords_bytes[idx+1] {
    //                         is_double_sub = false;
    //                     }
    //                 },
    //                 4 => {
    //                     if passwords_bytes[idx -1] == passwords_bytes[idx] {
    //                         is_double_sub = false;
    //                     }
    //                 },
    //                 _ => {
    //                     if passwords_bytes[idx+2] == passwords_bytes[idx+1] ||
    //                     passwords_bytes[idx-1] == passwords_bytes[idx] {
    //                         is_double_sub = false;
    //                     }
    //                 }
    //             }
    //             is_double = is_double_sub;
    //         }
            
    //         if num1 > num2 {
    //             return false
    //         }
    //     }
    //     is_double
    // }
}

impl Q4 {
    pub fn new() -> Self {
        Q4 {}
    }

    fn part1(&mut self) {
        let passes = (272091..=815432).filter(|&int| {
            let mut c = Container::new(int);
            c.check_password()}).count();
        // 481 
        println!("{:?}", passes);
    }

    fn part2(&mut self) {
        
    }

}

impl Runner for Q4 {
    fn part1(&mut self) {
        self.part1()
    }
    
    fn part2(&mut self) {
        self.part2()
    }
}


#[cfg(test)]
mod test{
    use crate::q4::Container;

    #[test]
    fn Q4() {
        let mut c = Container::new(112233);
        assert_eq!(true, c.check_password());
    }
    #[test]
    fn Q4_1() {
        let mut c = Container::new(123444);
        assert_eq!(false, c.check_password());
    }
    #[test]
    fn Q4_2() {
        let mut c = Container::new(111122);
        assert_eq!(true, c.check_password());
    }
}