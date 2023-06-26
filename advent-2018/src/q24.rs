use advent_2018::{Runner, get_file};


// unit 
// number of units
// hp
// atk
// atk-type
// initiative
// Option<Weakness, immunity>
// effective power: number of units * atk 

// target selection
// each grp selects one target
//  1. decending order of effective power, if tie, high initiative 
//  2. select enemy grp that will take the most dmg
//     after accouting weakness or immunity
//      - grp with largest effective power, if tie, highest initiative
//  3. if cannot deal any damage, then choose None
//  4. only one atk grp can dmg one def grp

// attacking
// 1. decreasing order of initiative

fn parse_number(num:&str) -> i32 {
    num.parse().unwrap()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Groups {
    units: Vec<Unit>
}

impl Groups {
    fn new(lines: &str) -> Self {
        let mut units = vec![];
        lines.lines().skip(1).enumerate().for_each(|(idx, line)| {
            println!("{:?}", line);
            match line.contains("(") {
                true => {
                    let id = idx as i32;
                    let data: Vec<_> = line.split(|p| p == '(' || p == ')').collect();
                    let (first, second, third) = (data[0],data[1],data[2]);
                    let first: Vec<_>= first.split_whitespace().collect();
                    let num_of_units: i32 = parse_number(first[0]);
                    let hp = parse_number(first[4]);
                    let third: Vec<_> = third.split_ascii_whitespace().collect();
                    let atk = parse_number(third[5]);
                    let atk_type = third[6].to_string();
                    let init = parse_number(third[10]);
                    let mut immune = Vec::new();
                    let mut weak = Vec::new();
                    match second.split_once(';') {
                        Some((im,wk)) => {
                            let (_, im) = im.split_once(" to ").unwrap();
                            immune= im.split(|c| c== ' ' || c == ',').map(String::from).collect(); 
                            let (_, wk) = wk.split_once(" to ").unwrap();
                            weak = wk.split(|c| c== ' ' || c == ',').map(String::from).collect(); 
                            
                        },
                        None => {
                            match second.contains("weak") {
                                true => {
                                    let (_, wk) = second.split_once(" to ").unwrap();
                                    weak = wk.split(|c| c== ' ' || c == ',').map(String::from).collect(); 
                                }
                                false => {
                                    let (_, im) = second.split_once(" to ").unwrap();
                                    immune = im.split(|c| c== ' ' || c == ',').map(String::from).collect(); 
                                }
                            }
                        }
                    }
                    units.push(Unit {atk,hp,id,init,num_of_units,atk_type,immune,weak})
                }
                false => {
                    let line: Vec<_>= line.split_whitespace().collect();
                    let id = idx as i32;
                    let num_of_units = parse_number(line[0]);
                    let hp = parse_number(line[4]);
                    let atk = parse_number(line[12]);
                    let atk_type = line[13].to_string();
                    let weak = Vec::new();
                    let immune = Vec::new();
                    let init = parse_number(line[17]);
                    units.push(Unit { id, num_of_units, hp, atk, atk_type, weak, immune, init })
                }
            }
            
            // let (im, wk) = second.split_once(';').unwrap();
        });
        Self { units }
    }

    
}

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
struct Unit {
    id: i32,
    num_of_units: i32,
    hp: i32,
    atk: i32,
    atk_type: String,
    weak: Vec<String>,
    immune: Vec<String>,
    init: i32,
}

impl Unit {
    fn effective_power(&self) -> i32 {
        self.atk * self.num_of_units
    }

    fn dmg_to(&self, other: &Unit) -> i32{
        if other.weak.contains(&self.atk_type) {
            self.effective_power() * 2
        } else if other.immune.contains(&self.atk_type) {
            0
        } else {
            self.effective_power()
        }
    }
}    

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Simulator {
    immune: Groups,
    infection: Groups
}

impl Simulator {
    fn new() -> Self {
        let content = get_file("src/input/q24.txt").unwrap();    
        let (first, second) = content.split_once("\r\n\r\n").unwrap(); 
        // println!("{:?}", lines[0]);
        let immune = Groups::new(first);
        let infection = Groups::new(second);
        Self {immune, infection}
    }

    fn target_selection(&mut self) {

    }

    fn attack_phase(&mut self) {
        
    }

    fn battle(&mut self) {
        while !(self.immune.units.is_empty() && self.infection.units.is_empty()) {
            // let mut attacks = vec![];
            self.immune.units.sort_by(|x,y| y.effective_power().cmp(&x.effective_power()).then(y.init.cmp(&x.init)));
            self.infection.units.sort_by(|x,y| y.effective_power().cmp(&x.effective_power()).then(y.init.cmp(&x.init)));

            let mut attacks = vec![];
            self.immune.units.iter().for_each(|im| {
                let mut dmgs_to_infection = vec![];
                self.infection.units.iter().for_each(|inf| {
                    if im.dmg_to(inf) > 0 {
                        dmgs_to_infection.push((inf.id, im.dmg_to(inf), inf.init))
                    }
                });
                dmgs_to_infection.sort_by(|x,y|y.1.cmp(&x.1).then(y.2.cmp(&x.2)));
                attacks.push((im.id, dmgs_to_infection));
            });

            self.infection.units.iter().for_each(|inf| {
                let mut dmg_to_imnune = vec![];
                self.immune.units.iter().for_each(|im| {
                    dmg_to_imnune.push((im.id, inf.dmg_to(im), im.init))
                });
                dmg_to_imnune.sort_by(|x,y|y.1.cmp(&x.1).then(y.2.cmp(&x.2)));
                attacks.push((inf.id, dmg_to_imnune));
            });            
            
            let mut seen = HashSet::new();
            attacks.iter().for_each(|attack| {

            })
            // });
        }
    }
}


pub struct Q24 {

}

impl Q24 {
    pub fn new() -> Self {
        Q24 {}
    }

    //    // 24725 dmg to m g 1 
    //    println!("{:?}", (989  - 84)  * 25);
    //    println!("{:?}",((3769506 - 22625) / 4706) );
    //    println!("{:?}", ((3769506 - 22625) % 4706) );
    //    println!("{:?}", 801 -797);
    //    println!("{:?}", (100 -75 )/ 10)
    //    // println!("{:?}", 107640 );
    //    // println!("{:?}", 24725);
    //    // println!("{:?}", (989 - 84)  * 25 );
    //    // println!("{:?}", 801 * 116 );
    //    // println!("{:?}", (92916 - 22625) / 4706   );
    //    // println!("{:?}", );
    fn part1(&mut self) {
        let mut simulator = Simulator::new();
        println!("{:?}", simulator);
        simulator.battle();
    }

    fn part2(&mut self) {
        
    }

}

impl Runner for Q24 {
    fn run(&mut self) {
        self.part1();
    }
}


#[cfg(test)]
mod test{
    #[test]
    fn Q24() {
        assert_eq!(1, 1);
    }
}