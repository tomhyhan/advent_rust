use std::collections::{HashMap, VecDeque, HashSet};

use advent_2020::{Runner, get_file};
use regex::Regex;
use once_cell::sync::Lazy;

#[derive(Debug)]
struct Ticket {
    my_ticket: Vec<i32>,
    nearby_tickets: Vec<Vec<i32>>,
    ranges: HashMap<String, Vec<Vec<i32>>> 
}

impl Ticket {
    fn new() -> Self {
        let content = get_file("src/input/q16.txt").unwrap();
        let info: Vec<_>= content.split("\r\n\r\n").collect();
        let (range_info, my_ticket_info, nearby_info) = (info[0],info[1],info[2]);

        // range
        let mut ranges = HashMap::new(); 
        range_info.lines().for_each(|line|{
            static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d+").unwrap());

            let (class, _) = line.split_once(": ").unwrap();
            let nums: Vec<i32> = RE.find_iter(line).map(|mat|{
                mat.as_str().parse::<i32>().unwrap()
            }).collect();
            let r = vec![vec![nums[0],nums[1]],vec![nums[2],nums[3]]];
            ranges.insert(class.to_string(), r);
        });

        // my ticket
        let my_ticket = my_ticket_info.lines().nth(1).unwrap().split(',').map(|val| val.parse::<i32>().unwrap()).collect();

        // near tcikets
        let nearby_tickets = nearby_info.lines().skip(1).map(|line| {
            line.split(',').map(|val| val.parse::<i32>().unwrap()).collect()
        }).collect();
        
        Self {my_ticket,nearby_tickets,ranges}
    }

    fn find_invalid_numbers(&self) -> Vec<usize> {
        let mut invalids = vec![];
        self.nearby_tickets.iter().enumerate().for_each(|(idx, ticket)| {
            ticket.iter().for_each(|num|{
                let mut found = false;
                for ranges in self.ranges.values() {
                    if (ranges[0][0]..=ranges[0][1]).contains(num) || (ranges[1][0]..=ranges[1][1]).contains(num) {
                        found = true;
                        break
                    }
                }            
                if !found {
                    invalids.push(idx);
                }
            })
        });
        invalids
    }

    fn find_matching_class(&self) {
        let invalids = self.find_invalid_numbers();
        let class_set = self.ranges.keys().cloned().collect::<HashSet<String>>();

        let mut class_matches = vec![];
        for i in 0..self.my_ticket.len() {
            let mut current_set= class_set.clone();
            for (idx, ticket) in self.nearby_tickets.iter().enumerate() {
                if invalids.contains(&idx) {
                    continue
                }
                let num = ticket[i];
                let mut valid_classes = HashSet::new();
                for class in self.ranges.keys() {
                    if !current_set.contains(class) {
                        continue
                    }
                    if self.is_valid(num,class) {
                        valid_classes.insert(class.clone());
                    }
                }
                current_set = current_set.intersection(&valid_classes).map(|s| s.clone()).collect();
            }
            class_matches.push(current_set);
        }
        let mut class_matches_order: Vec<_>= class_matches.into_iter().enumerate().map(|c|c).collect();

        class_matches_order.sort_by_key(|set|set.1.len());

        for i in (1..class_matches_order.len()).rev() {
            class_matches_order[i].1 =  class_matches_order[i].1.difference(&class_matches_order[i-1].1).cloned().collect::<HashSet<_>>();
        }
        println!("{:?}", class_matches_order);
        let de = self.my_ticket[16] as i64;
        let de1 = self.my_ticket[1] as i64;
        let de2 = self.my_ticket[14] as i64;
        let de3 = self.my_ticket[17] as i64;
        let de4 = self.my_ticket[2] as i64;
        let de5 = self.my_ticket[6] as i64;
        println!("{:?}", de * de1 * de2 * de3 * de4* de5);
    }

    fn is_valid(&self, val: i32, class: &String) -> bool {
        let ranges = self.ranges.get(class).unwrap();
        if (ranges[0][0]..=ranges[0][1]).contains(&val) || (ranges[1][0]..=ranges[1][1]).contains(&val) {
            return true
        }
        false
    }
}

pub struct Q16 {

}

impl Q16 {
    pub fn new() -> Self {
        Q16 {}
    }

    fn part1(&mut self) {
        let ticket = Ticket::new();
       println!("{:?}",  ticket.find_invalid_numbers());
    }

    fn part2(&mut self) {
        let ticket = Ticket::new();
        ticket.find_matching_class()
    }

}

impl Runner for Q16 {
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
    fn Q16() {
        assert_eq!(1, 1);
    }
}

// solution 1
// fn find_matching_class(&self) {
//     let invalids = self.find_invalid_numbers();
//     let mut stack = VecDeque::new();

//     for class in self.ranges.keys() {
//         stack.push_back((vec![], 0, class.clone()))
//     }

//     while !stack.is_empty() {
//         let (mut classes, idx, class) = stack.pop_front().unwrap();

//         classes.push(class.clone());
//         if classes.len() == self.ranges.keys().len() {
//             println!("{:?}", classes);
//             break
//         }

//         let mut not_in_range = false;
//         for (i, ticket) in self.nearby_tickets.iter().enumerate() {
//             if invalids.contains(&i) {
//                 continue
//             }
//             let ranges = self.ranges.get(&class).unwrap();
//             let val = ticket[idx];
//             if (ranges[0][0]..=ranges[0][1]).contains(&val) || (ranges[1][0]..=ranges[1][1]).contains(&val) {
//                 continue
//             }
//             not_in_range = true;
//             break
//         }
//         if not_in_range {
//             continue
//         }
        
//         for other_class in self.ranges.keys() {
//             if classes.contains(other_class) {
//                 continue;
//             }
//             stack.push_back((classes.clone(), idx +1, other_class.clone()));
//         }
//     }
// }