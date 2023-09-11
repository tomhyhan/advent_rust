use advent_2020::Runner;

pub struct Q25 {

}

impl Q25 {
    pub fn new() -> Self {
        Q25 {}
    }

    fn part1(&mut self) {
        let card_pub = 17773298;
        let door_pub = 15530095;
        let card_loop = get_loop(card_pub);
        let door_loop = get_loop(door_pub);
        println!("{card_loop}, {door_loop}");
        let card_secret = get_secret(door_pub, card_loop);
        let door_secret = get_secret(card_pub, door_loop);
        println!("{:?} {}", card_secret, door_secret)
    }

    fn part2(&mut self) {

    }

}

fn get_secret(key: i64, loops: i64) -> i64 {
    let mut value = 1;
    let subject_val = key;

    for _ in 0..loops {
        value = value * subject_val;
        value = value % 20201227;
    }
    value
}

fn get_loop(key: i64) -> i64 {
    let mut value = 1;
    let subject_val = 7;

    let mut loops = 0;
    while value != key {
        value = value * subject_val;
        value = value % 20201227;
        loops += 1
    }
    loops
}

impl Runner for Q25 {
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
    fn Q25() {
        assert_eq!(1, 1);
    }
}