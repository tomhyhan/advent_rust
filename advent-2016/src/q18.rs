use crate::Runner;
use itertools::{Itertools, Position};
pub struct Q18 {

}

#[derive(Debug, Clone, PartialEq)]
enum Tile {
    Safe,
    Trap
}

#[derive(Debug, Clone)]
struct Room {
    tiles : Vec<Vec<Tile>>,
    idx: usize
}

impl Room {
    fn new(starting_tile: String) -> Self {
        
        let mut tiles = Vec::new();
        let starting_tile = starting_tile.chars().map(|c| {match c {
            '.' => Tile::Safe,
            '^' => Tile::Trap,
            _ => panic!("invalid tile")
        }}).collect::<Vec<Tile>>();
        tiles.push(starting_tile);
        Room {
            tiles,
            idx : 0,
        }
    }

    fn is_trap(&self, tiles: [&Tile;3]) -> bool {
        // println!("{} - {:?}", self.idx, tiles);
        let condition1 = *tiles[0]  == Tile::Trap && *tiles[1]  == Tile::Trap  && *tiles[2]  == Tile::Safe;
        let condition2 = *tiles[0]  == Tile::Safe && *tiles[1]  == Tile::Trap  && *tiles[2]  == Tile::Trap;
        let condition3 = *tiles[0]  == Tile::Trap && *tiles[1]  == Tile::Safe  && *tiles[2]  == Tile::Safe;
        let condition4 = *tiles[0]  == Tile::Safe && *tiles[1]  == Tile::Safe  && *tiles[2]  == Tile::Trap;
        if condition1 || condition2 || condition3 || condition4 {
            return true
        }
        false
    }

}
impl Iterator for Room {
    type Item = (usize, Vec<Tile>);
    fn next(&mut self) -> Option<Self::Item> {
        let current_tile = self.tiles[self.idx].clone();
        
        let next_tile : Vec<Tile> = self.tiles[self.idx]
                            .clone()
                            .iter()
                            .enumerate()
                            .with_position()
                            .map(|line| {
                                match line {
                                    Position::First((idx, _)) => {
                                        if self.is_trap([&Tile::Safe, &current_tile[idx], &current_tile[idx+1]]) {
                                            return Tile::Trap;
                                        }
                                        Tile::Safe
                                    }
                                    Position::Last((idx, _)) => {
                                        if self.is_trap([&current_tile[idx-1], &current_tile[idx], &Tile::Safe]) {
                                            return Tile::Trap;
                                        }
                                        Tile::Safe  
                                    }
                                    Position::Middle((idx, _)) => {
                                        if self.is_trap([&current_tile[idx-1], &current_tile[idx], &current_tile[idx+1]]) {
                                            return Tile::Trap;
                                        }
                                        Tile::Safe 
                                    },
                                    _ => panic!("unknown position!")
                                }
                            })
                            .collect();
        // println!("current - {:?}", current_tile);
        // println!("next - {:?}", next_tile);
        self.idx += 1;
        self.tiles.push(next_tile.clone());
        // println!("{:?}",self.tiles);
        Some((self.idx, next_tile))
    }

    
}

impl Q18 {
    pub fn new() -> Self {
        Q18{}
    }

    fn part1(&self, tile: String, rounds: usize) -> usize{
        let room = Room::new(tile);
        let mut rooms = room.into_iter();
        while let Some(tile) = rooms.next()  {
            if tile.0 >= rounds - 1 {
                break
            }
        }
        rooms.tiles.iter().map(|tile| tile.iter().filter(|p| **p == Tile::Safe).count()).sum()
    }

}

impl Runner for Q18 {
    fn run(&mut self) -> () {
        println!("tiles - {}", self.part1( "^.^^^.^..^....^^....^^^^.^^.^...^^.^.^^.^^.^^..^.^...^.^..^.^^.^..^.....^^^.^.^^^..^^...^^^...^...^.".to_string(), 40));
    }
}



#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn tiles_return_38safe_tiles()  {
        let q18 = Q18::new();
        assert_eq!(q18.part1(".^^.^.^^^^".to_string(), 10), 38)
    }
}