use std::collections::{HashMap, HashSet};

fn get_divisors(houses: i32) {
    let mut num = 1;
    loop {
        if num == 1 {
            num += 1;
            continue;
        }

        let mut divisors = Vec::new();
        for i in 1..=(num as f64).sqrt() as i32 {
            if num % i == 0 {
                divisors.push(i);
                if num / i > i {
                    divisors.push(num / i);
                }
            }
        }
        // println!("{:?}", divisors);
        divisors = divisors.into_iter().filter(|n| n * 50 >= num).collect();
        if divisors.iter().sum::<i32>() * 11 >= houses {
            println!("{}", divisors.iter().count());
            println!("{:?}", divisors);
            println!("{}", num);
            break;
        }
        // println!("{}", divisors.iter().sum::<i32>() * 10);
        num += 1;
        // println!("{num}")
        // if num == 5 {
        //     break;
        // }
    }
    println!("{num}")
}
fn main() {
    // 29000000
    // 2
    // 5 {8, 16, 1, 4, 2}
    // 16 {3, 10, 30, 20, 4, 1, 2, 5, 6, 12, 8, 15, 40, 24, 60, 120}
    // 28 {2, 30, 4, 192, 3, 5, 120, 96, 6, 160, 480, 240, 12, 16, 80, 48, 15, 1, 60, 40, 64, 20, 24, 32, 8, 320, 960, 10}
    // 60
    //  718200 too  high
    let houses = 29000000;
    get_divisors(houses);
    println!("{}", (12 as f32).sqrt());
}
