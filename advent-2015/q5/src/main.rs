use std::fs;

fn main() {
    let path = "input.txt";
    let context = fs::read_to_string(path).expect("Error");
    println!("{context}");

    let mut nice = Vec::new();
    let lines: Vec<_> = context.split("\n").collect();
    println!("{lines:?}");

    lines.iter().for_each(|line| is_nice(line, &mut nice));
    println!("{nice:?}");
    let nices = nice.iter().filter(|n| n == &&true).count();
    println!("{nices}");
}

fn is_nice(line: &str, nice: &mut Vec<bool>) {
    let length = line.len();
    let mut double = false;
    let mut bwt = false;
    for i in 0..length {
        let current_char = &line[i..i + 1];
        if i < length - 1 {
            let substring = &line[i..i + 2];
            let other = &line[i + 2..length];
            if other.contains(substring) {
                double = true;
            }
        }
        if i < length - 2 {
            let third_char = &line[i + 2..i + 3];
            if current_char == third_char {
                bwt = true;
            }
        }
    }

    if double && bwt {
        nice.push(true)
    } else {
        nice.push(false)
    }
    //
}

// fn is_nice(line: &str, nice: &mut Vec<bool>) {
//     let length = line.len();
//     let mut double = false;
//     let mut bwt = false;
//     for i in 0..length {
//         let current_char = &line[i..i+1];
//         if i < length - 1 {
//             let substring = &line[i..i+2];
//             let other = &line[i+2..length];
//             if other.contains(substring){
//                 double = true;
//             }
//         }
//         if i < length - 2 {
//             let third_char = &line[i+2..i+3];
//             if current_char == third_char {
//                 bwt = true;
//             }
//         }
//     }

//     if double && bwt {
//         nice.push(true)
//     } else {
//         nice.push(false)
//     }

// }

// fn is_nice(line: &str, nice: &mut Vec<bool>) {
//     let length = line.len();
//     let mut vowels = 0;
//     let mut double = 0;
//     for i in 0..length {
//         let current_char = &line[i..i+1];
//         if i < length - 1 {
//             let substring = &line[i..i+2];
//             let next_char = &line[i+1..i+2];
//             match substring {
//                 "ab" | "cd" | "pq" | "xy" => {
//                     nice.push(false);
//                     return
//                 },
//                 _ => ()
//             }
//             if current_char == next_char{
//                 double += 1
//             }
//         }
//         if let "a" | "e" | "i" | "o" | "u"  = current_char {
//             vowels += 1
//         }
//     }

//     if vowels >= 3 && double >= 1 {
//         nice.push(true)
//     } else {
//         nice.push(false)
//     }

// }
