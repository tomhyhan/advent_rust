use std::{collections::{HashMap, VecDeque, HashSet}, str::FromStr};

use advent_2018::{Runner, get_file};

pub struct Q6 {

}

#[derive(Debug)]
struct Area {
    points: HashMap<usize, Point>,
    max_row: i32,
    min_row: i32,
    max_col: i32,
    min_col: i32,
}

impl Area {
    fn new() -> Self {
        let content = get_file("src/input/q6.txt").unwrap();
        let mut points = HashMap::new();
        let mut max_row = i32::MIN;
        let mut min_row = i32::MAX;
        let mut max_col = i32::MIN;
        let mut min_col = i32::MAX;

        content.lines().enumerate().for_each(|(coord, line)| {
            let (col, row) = line.split_once(", ").unwrap();
            let row = row.parse().unwrap();
            let col = col.parse().unwrap();
            max_row = max_row.max(row); 
            min_row = min_row.min(row);
            max_col = max_col.max(col); 
            min_col = min_col.min(col); 
            points.insert(coord, Point{row, col});
        });
        Self { points,max_col,max_row,min_col,min_row }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point {
    row: i32,
    col: i32
}

impl Q6 {
    pub fn new() -> Self {
        Q6 {}
    }

    fn part1(&mut self) {
        let area = Area::new();
        let mut area_cover: HashMap<(i32,i32), Vec<usize>> = HashMap::new();
        for row in area.min_row..=area.max_row {
            for col in area.min_col..=area.max_col {
                calc_distance(row,col,&mut area_cover, &area.points)
            }
        }

        let mut infinites = HashSet::new();
        
        for row in area.min_row..=area.max_row {
            for col in area.min_col..=area.max_col {
                let row_border = row == area.min_row || row == area.max_row;
                let col_border = col == area.min_col || col == area.max_col;
                let is_border = row_border || col_border;
                if is_border {
                    let id = area_cover.get(&(row,col)).unwrap();
                    if id.len() > 1 {
                        continue
                    }
                    for id in id {
                        infinites.insert(id);
                    }
                }
            }
        }

        let mut areas = HashMap::new();
        for row in area.min_row..=area.max_row {
            for col in area.min_col..=area.max_col {
                let id = area_cover.get(&(row,col)).unwrap();
                if id.len() > 1 {
                    continue
                }
                areas.entry(*id.last().unwrap()).and_modify(|cnt| *cnt+= 1).or_insert(1);
            }
        }
        println!("{:?}", areas.iter().filter(|(key,_)| !infinites.contains(key)).max_by(|x,y|x.1.cmp(&y.1)).unwrap());
    }
    fn part2(&mut self) {
        let area = Area::new();

        let mut size = 0;
        for row in area.min_row..=area.max_row {
            for col in area.min_col..=area.max_col {
                let mut total_distance = 0;
                for (_, point) in area.points.iter() {
                    let distance = (point.row - row).abs() + (point.col - col).abs();
                    total_distance += distance
                }
                if total_distance < 10000 {
                    size += 1;
                }
            }
        }
        println!("{:?}", size);
    }

}

fn calc_distance(row:i32, col: i32, area_cover: &mut HashMap<(i32,i32), Vec<usize>>, points: &HashMap<usize, Point>) {
    let mut smallest_distances: Vec<(usize, i32)> = vec![];
    
    for (id, point) in points {
        let distance = (point.row - row).abs() + (point.col - col).abs();
        if smallest_distances.is_empty() {
            smallest_distances.push((*id, distance));
        } else if smallest_distances[smallest_distances.len() - 1].1 > distance {
            smallest_distances.clear();
            smallest_distances.push((*id, distance));
        } else if smallest_distances[smallest_distances.len() - 1].1 == distance {
            smallest_distances.push((*id, distance));
        } else {
            continue
        }
    }

    area_cover.insert((row,col), smallest_distances.iter().map(|(id,_)|*id).collect());

}

impl Runner for Q6 {
    fn run(&mut self) {
        // self.part1()
        self.part2()
    }
}


#[cfg(test)]
mod test{
    #[test]
    fn Q6() {
        assert_eq!(1, 1);
    }
}