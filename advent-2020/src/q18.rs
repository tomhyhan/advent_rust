use std::collections::HashMap;

use advent_2020::{Runner, get_file};

struct Math {
    expressions: Vec<Vec<char>>,
    brackets: HashMap<usize,usize>
}

impl Math {
    fn new() -> Self {
        let content = get_file("src/input/q18.txt").unwrap();
        let mut brackets = HashMap::new();
        let expressions = content.lines().map(|line| {
            let mut tokens = vec![];
            let mut stack = vec![];
            for (idx, c) in line.chars().enumerate() {
                if c == ' ' {
                    continue
                }
                if c == '(' {
                    stack.push(idx)
                } else if c == ')' {
                    let open_idx = stack.pop().unwrap();
                    let close_idx = idx;
                    brackets.insert(open_idx, close_idx);
                }
                tokens.push(c);
            };
            tokens
        }).collect();

        Self {brackets,expressions}
    }

    fn sum_of_evaluations(&self) -> i64 {
        let mut sum = 0;
        for expression in self.expressions.iter() {
            sum += self.evaludate_expression(expression)
        }
        sum
    }

    fn evaludate_expression(&self, tokens: &Vec<char>) -> i64 {
        let mut calc: HashMap<char, fn(i64, i64) -> i64> = HashMap::new();
        calc.insert('+', plus);
        calc.insert('*', multiply);        

        let mut current = None;
        let mut stack = vec![];        
        let mut num_stack = vec![];

        for char in tokens.iter() {
            match char {
                '+' | '*' => {
                    stack.push(char)
                }
                '(' => {
                    if current.is_some() {
                        num_stack.push(current);
                    }
                    current = None
                }
                ')' => {
                    if let Some(val) = current {
                        if !stack.is_empty() {
                            let operator = stack.pop().unwrap(); 
                            let prev = num_stack.pop().unwrap().unwrap();
                            current = Some(calc.get(operator).unwrap()(prev, val));
                        } else {
                            current = Some(val)
                        }
                    }
                }
                c => {
                    if c.is_digit(10) {
                        if current.is_none() {
                            current = Some( char::to_digit(*c, 10).unwrap() as i64);
                            continue;
                        } else if let Some(val) = current {
                            let num = char::to_digit(*c, 10).unwrap() as i64;
                            let operator = stack.pop().unwrap(); 
                            current = Some(calc.get(operator).unwrap()(val, num));
                        }
                    }   
                }
            }                    
        }
        current.unwrap()
    }
}


fn plus(x:i64, y:i64) -> i64{
    x + y
}

fn multiply(x:i64, y:i64) -> i64{
    x * y
}

pub struct Q18 {
}

impl Q18 {
    pub fn new() -> Self {
        Q18 {}
    }

    fn part1(&mut self) {
        let math = Math::new();
        println!("{:?}", math.sum_of_evaluations())
    }

    fn part2(&mut self) {
        
    }

}

impl Runner for Q18 {
    fn part1(&mut self) {
        self.part1()
    }
    
    fn part2(&mut self) {
        self.part2()
    }
}


#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_expression1() {
        let math = Math::new(); 
        math.sum_of_evaluations();
        assert_eq!(1,2)
    }
}