use advent_2020::{Runner, get_file};


struct TobogganDatabase {
    passwords: Vec<PasswordInfo>
}

impl TobogganDatabase {
    fn new() -> Self {
        let content = get_file("src/input/q2.txt").unwrap();
        let passwords = content.lines().map(|line|{
            let (policy, password) = line.split_once(": ").unwrap();
            let (range, chr) = policy.split_once(" ").unwrap();
            let (lower, higher) = range.split_once("-").unwrap();
            PasswordInfo {
                password: password.to_string(),
                policy: (lower.parse::<usize>().unwrap(),higher.parse::<usize>().unwrap(),chr.chars().next().unwrap())
            }
        }).collect();
        Self { passwords }
    }

    fn valid_passwords(&self) -> usize{
        self.passwords.iter().filter(|password| password.check_password()).count()
    }

    fn new_valid_passwords(&self) -> usize {
        self.passwords.iter().filter(|password: &&PasswordInfo| password.new_check_password()).count()
    }
}

#[derive(Debug)]
struct PasswordInfo {
    password: String,
    policy: (usize,usize,char)
}

impl PasswordInfo {
    fn check_password(&self) -> bool {
        let count = self.password.chars().filter(|c| *c == self.policy.2).count();
        if count >= self.policy.0 && count <= self.policy.1 {
            return true
        }
        false
    }

    fn new_check_password(&self) -> bool {
        let vec: Vec<char> = self.password.chars().collect();
        let first_match = vec[self.policy.0-1] == self.policy.2;
        let second_match = vec[self.policy.1-1] == self.policy.2;
        if (first_match && !second_match) || (!first_match && second_match) {
            return true
        }
        false
    }
}
pub struct Q2 {

}

impl Q2 {
    pub fn new() -> Self {
        Q2 {}
    }

    fn part1(&mut self) {
        let db = TobogganDatabase::new();
        println!("{:?}", db.valid_passwords());
    }

    fn part2(&mut self) {
        let db = TobogganDatabase::new();
        println!("{:?}", db.new_valid_passwords());
    }

}

impl Runner for Q2 {
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
    fn Q2() {
        assert_eq!(1, 1);
    }
}