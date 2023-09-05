use advent_2020::{get_file, Runner};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Default, Clone)]
struct Image {
    tile: Vec<Vec<char>>,
}

impl Image {
    fn new(tile: Vec<Vec<char>>) -> Self {
        Self { tile }
    }

    fn rotate(&mut self) {
        for i in 0..self.tile.len() {
            for j in i + 1..self.tile[0].len() {
                let temp = self.tile[i][j];
                self.tile[i][j] = self.tile[j][i];
                self.tile[j][i] = temp;
            }
        }
        self.flip();
    }

    fn flip(&mut self) {
        for tile in self.tile.iter_mut() {
            tile.reverse();
        }
    }

    fn rotations(&self) -> Vec<Image> {
        let mut rotations = vec![];
        let mut tile = self.clone();
        for _ in 0..4 {
            rotations.push(tile.clone());
            tile.rotate()
        }
        tile.flip();
        for _ in 0..4 {
            rotations.push(tile.clone());
            tile.rotate()
        }
        rotations
    }

    fn top(&self) -> Vec<char> {
        self.tile[0].clone()
    }

    fn bottom(&self) -> Vec<char> {
        self.tile[self.tile.len() - 1].clone()
    }

    fn left(&self) -> Vec<char> {
        self.tile.iter().map(|tile| tile[0]).collect()
    }

    fn right(&self) -> Vec<char> {
        self.tile.iter().map(|tile| tile[tile.len() - 1]).collect()
    }
}

struct CameraArray {
    images: HashMap<i64, Image>,
}

impl CameraArray {
    fn new() -> Self {
        let content = get_file("src/input/q20.txt").unwrap();
        let tiles: Vec<_> = content.split("\r\n\r\n").collect();
        let mut images = HashMap::new();
        tiles.iter().for_each(|tile| {
            lazy_static! {
                static ref RE: Regex = Regex::new(r"\d+").unwrap();
            }
            let mut tile_lines = tile.lines();
            let first_line = tile_lines.next().unwrap();
            let num = RE.find(first_line).unwrap().as_str();
            let id = num.parse::<i64>().unwrap();
            let tile: Vec<Vec<char>> = tile_lines.map(|line| line.chars().collect()).collect();
            images.insert(id, Image { tile });
        });
        Self { images }
    }

    fn images(&mut self) {
        let len = (self.images.len() as f64).sqrt() as usize;
        let mut tiles: Vec<Vec<Image>> = vec![vec![Image::default(); len]; len];
        let mut tiles_ids: Vec<Vec<_>> = vec![vec![0; len]; len];
        let mut seen = HashSet::new();
        self.fill_tiles(0, 0, &mut tiles, &mut seen, len, &mut tiles_ids);

        // println!(
        //     "{:?}",
        //     tiles_ids[0][0]
        //         * tiles_ids[0][len - 1]
        //         * tiles_ids[len - 1][0]
        //         * tiles_ids[len - 1][len - 1]
        // );
        let joined_image = self.remove_and_join(tiles);
        self.find_sea_monter(joined_image);
    }

    fn remove_and_join(&self, tiles: Vec<Vec<Image>>) -> Image {
        let mut joined = Image::new(vec![vec![]; 8 * tiles.len()]);
        for row in 0..tiles.len() {
            for col in 0..tiles[0].len() {
                let image = &tiles[row][col].tile;
                for r in 1..image.len() - 1 {
                    for c in 1..image[0].len() - 1 {
                        let char = image[r][c];
                        let cr = (r - 1) + row * 8;
                        joined.tile[cr].push(char);
                    }
                }
            }
        }
        joined
    }

    fn sea_monster_locations(&self) -> Vec<(usize, usize)> {
        let sea =
            "                  # \r\n#    ##    ##    ###\r\n #  #  #  #  #  #   ".to_string();
        let mut locations = vec![];
        for (row, line) in sea.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                if c == '#' {
                    locations.push((row, col));
                }
            }
        }
        locations
    }

    fn find_sea_monter(&self, image: Image) {
        for rotation in image.rotations() {
            let mut sea_monsters = HashSet::new();
            let sea_monster: Vec<(usize, usize)> = self.sea_monster_locations();
            for row in 0..rotation.tile.len() {
                for col in 0..rotation.tile[0].len() {
                    if self.is_sea_monster(row, col, &rotation) {
                        for (r, c) in sea_monster.iter() {
                            let sr = row + r;
                            let sc = col + c;
                            sea_monsters.insert((sr, sc));
                        }
                    }
                }
            }
            if !sea_monsters.is_empty() {
                println!(
                    "len - {:?}",
                    rotation
                        .tile
                        .iter()
                        .map(|t| t.iter().filter(|&&c| c == '#').count())
                        .sum::<usize>()
                        - sea_monsters.len()
                );
                return;
            }
        }
    }

    fn is_sea_monster(&self, row: usize, col: usize, image: &Image) -> bool {
        let sea_monster: Vec<(usize, usize)> = self.sea_monster_locations();
        for (r, c) in sea_monster.into_iter() {
            let sr = row + r;
            let sc = col + c;
            if sr >= image.tile.len() || sc >= image.tile[0].len() || image.tile[sr][sc] != '#' {
                return false;
            }
        }
        true
    }

    fn check_left(&self, current: &Image, other: &Image) -> bool {
        if current.left() == other.right() {
            return true;
        }
        false
    }

    fn check_top(&self, current: &Image, other: &Image) -> bool {
        if current.top() == other.bottom() {
            return true;
        }
        false
    }

    fn fill_tiles(
        &self,
        row: usize,
        col: usize,
        tiles: &mut Vec<Vec<Image>>,
        seen: &mut HashSet<i64>,
        len: usize,
        tiles_ids: &mut Vec<Vec<i64>>,
    ) -> bool {
        if row >= len || col >= len {
            return true;
        }

        for (id, image) in self.images.iter() {
            if !seen.insert(*id) {
                continue;
            }
            // println!("{}, {}", row,col);
            for rotation in image.rotations() {
                tiles[row][col] = rotation.clone();
                tiles_ids[row][col] = *id;
                let mut left = true;
                let mut top = true;

                if row > 0 {
                    top = self.check_top(&rotation, &tiles[row - 1][col]);
                }

                if col > 0 {
                    left = self.check_left(&rotation, &tiles[row][col - 1])
                }

                if left && top {
                    let nc = (col + 1) % len;
                    // let nr = ((col+1)/len) + row;
                    let nr = (len * row + col + 1) / len;
                    if self.fill_tiles(nr, nc, tiles, seen, len, tiles_ids) {
                        return true;
                    }
                }
                tiles[row][col] = Image::default();
                tiles_ids[row][col] = -1;
            }
            seen.remove(id);
        }

        false
    }
}

pub struct Q20 {}

impl Q20 {
    pub fn new() -> Self {
        Q20 {}
    }

    fn part1(&mut self) {
        let mut camera = CameraArray::new();
        camera.images();
    }

    fn part2(&mut self) {}
}

impl Runner for Q20 {
    fn part1(&mut self) {
        self.part1()
    }

    fn part2(&mut self) {
        self.part2()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn Q20() {
        let mut image = Image::new(vec![
            vec!['1', '2', '3'],
            vec!['4', '5', '6'],
            vec!['7', '8', '9'],
        ]);
        // image.rotate();
        // for t in image.tile {
        //     println!("{:?}", t);
        // };
        println!("{:?}", image.top());
        assert_eq!(1, 2);
    }
}
