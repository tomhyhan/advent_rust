use advent_2019::{Runner, get_file};
use itertools::Itertools;

struct ImageFormat {
    layers: Vec<Vec<u32>>,
    row: usize,
    col: usize
}
//  0 - black
//  1 - white
//  2 - transparent
impl ImageFormat {
    fn new(row: usize, col: usize) -> Self {
        let content = get_file("src/input/q8.txt").unwrap();
        let mut layers = vec![];
        let total_length = content.len();
        let num_of_layers = total_length / (row * col);
        println!("num_of_layers - {:?}", num_of_layers);
        for chars in &content.chars().chunks(total_length / num_of_layers) {
            let layer: Vec<_> = chars.into_iter().map(|c| c.to_digit(10).unwrap()).collect();
            layers.push(layer)
        }
        Self {layers, row, col}
    }

    fn find_fewest_0_digits(&self) -> &Vec<u32>{
        self.layers.iter().min_by(|x,y| x.iter().filter(|&&n| n == 0).count().cmp(&y.iter().filter(|&&n| n == 0).count())).unwrap()
    }

    fn multiply_num_of_0_and_1(&self, fewest_0: &Vec<u32>) -> usize{
        let ones = fewest_0.iter().filter(|&&n| n==1).count();
        let twos = fewest_0.iter().filter(|&&n| n==2).count();
        ones * twos
    }
    
    fn decode_image(&self) {
        let mut image = Vec::new();
        for row in 0..self.row {
            let image_col = vec![2;self.col];
            image.push(image_col)
        }
        for layer in self.layers.iter() {
            for (idx, num) in layer.iter().enumerate() {
                let pos_col = idx % self.col;
                let pos_row = idx / self.col;
                if image[pos_row][pos_col] != 2 {continue}
                if *num == 1 {
                    image[pos_row][pos_col] = *num;
                } else if *num == 0 {
                    image[pos_row][pos_col] = *num;
                }
            }
        }
        for line in image.iter() {
            println!("{:?}", line.iter().map(|num| {
                // for easy reading :p
                if *num == 0 {
                    " ".to_string()
                } else {
                    format!("{}",*num)
                }
            }).collect::<String>())
        }
    }
}

pub struct Q8 {

}

impl Q8 {
    pub fn new() -> Self {
        Q8 {}
    }

    fn part1(&mut self) {
        let image = ImageFormat::new(6,25);
        let fewest_0 = image.find_fewest_0_digits();
        println!("{:?}", fewest_0);
        println!("{:?}", image.multiply_num_of_0_and_1(fewest_0));
    }

    fn part2(&mut self) {
        let image = ImageFormat::new(6,25);
        image.decode_image()
    }

}

impl Runner for Q8 {
    fn part1(&mut self) {
        self.part1()
    }
    
    fn part2(&mut self) {
        self.part2()
    }
}


#[cfg(test)]
mod test{
    #[test]
    fn Q8() {
        assert_eq!(1, 1);
    }
}