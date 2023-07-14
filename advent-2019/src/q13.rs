use std::{collections::HashMap, thread, time};

use advent_2019::Runner;

use crate::int_program::Program;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Tile {
    Empty,
    Wall,
    Block,
    HorizontalPaddle,
    Ball
}

impl Tile {
    fn parse(id: i64) -> Tile {
        match id {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::HorizontalPaddle,
            4 => Tile::Ball,
            _ => panic!("invalid id")
        }
    }
}

struct Arcade {
    grid: HashMap<(i64,i64), Tile>,
    program: Program,
    min_row: i64,
    max_row: i64,
    min_col: i64,
    max_col: i64
}

impl Arcade {
    fn new(program: Program) -> Self {
        let grid = HashMap::new();
        let min_row = i64::MAX;
        let max_row = i64::MIN;
        let min_col = i64::MAX;
        let max_col = i64::MIN;
        Self {grid,program,min_row,
            max_row,
            min_col,
            max_col,}
    }

    fn draw_tile(&mut self) {
        let mut input = 0;
        let mut ball = None;
        let mut paddle = None;
        loop {
            let x = match self.program.run_program(input) {
                Some(x) => x,
                None => break
            };
            let y = self.program.run_program(input).unwrap();
            let tile = self.program.run_program(input).unwrap();
            // println!("{} {} {}", x , y , tile);
            if x == -1 && y == 0 {
                println!("{tile}");
                continue;
            }
            match Tile::parse(tile) {
                Tile::Ball => ball = Some(x),
                Tile::HorizontalPaddle => paddle = Some(x),
                _ => {}
            }  
            if let (Some(ball_pos),Some(paddle_pos)) = (ball, paddle) {
                if ball_pos > paddle_pos {
                    input = 1
                } else if ball_pos < paddle_pos {
                    input = -1
                } else {
                    input = 0
                }
            }
            
            // println!("tile - {:?}", tile);

            self.min_row = self.min_row.min(y);
            self.max_row = self.max_row.max(y);
            self.min_col = self.min_col.min(x);
            self.max_col = self.max_col.max(x);
            self.grid.insert((y,x), Tile::parse(tile));

            self.draw_screen();

            thread::sleep(time::Duration::from_millis(20));
        }
    }

    fn draw_screen(&self) {
        for row in self.min_row..=self.max_row {
            let mut s = String::new();
            for col in self.min_col..=self.max_col {
                if let Some(c) = self.grid.get(&(row,col)) {
                    match *c {
                        Tile::Ball => s.push('.'),
                        Tile::Block => s.push('#'),
                        Tile::Empty => s.push(' '),
                        Tile::HorizontalPaddle => s.push('_'),
                        Tile::Wall => s.push('|')
                    };
                } else {
                    s.push(' ')
                }
            }
            println!("{:?}", s);
        }
    } 


}

pub struct Q13 {

}

impl Q13 {
    pub fn new() -> Self {
        Q13 {}
    }
    
    fn part1(&mut self) { 
        let program = Program::new();
        let mut arcade = Arcade::new(program);
        arcade.draw_tile();
        println!("{:?}", arcade.grid.iter().filter(|(_, &v)| v == Tile::Block).count());
        // arcade.draw_screen();

 
    }

    fn part2(&mut self) {
        let mut program = Program::new();
        program.integers[0] = 2;
        let mut arcade = Arcade::new(program);
        arcade.draw_tile();
        // arcade.draw_tile();
        // arcade.draw_screen();

    }

}

impl Runner for Q13 {
    fn part1(&mut self) {
        // self.part1()
    }
    
    fn part2(&mut self) {
        self.part2()
    }
}


#[cfg(test)]
mod test{
    #[test]
    fn Q13() {
        assert_eq!(1, 1);
    }
}