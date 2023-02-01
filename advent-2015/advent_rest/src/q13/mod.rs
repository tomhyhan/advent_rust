use permutohedron;
use std::cmp;
use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    happiness: i32,
}

pub fn run_code() {
    let path = "q13.txt";
    let context = fs::read_to_string(path).expect("cannot read a file");

    let mut graph: HashMap<&str, Vec<Person>> = HashMap::new();
    context.lines().for_each(|line| parse(line, &mut graph));

    // println!("{:?}", graph);

    get_distances(&graph);
}

fn get_distances<'a>(graph: &HashMap<&'a str, Vec<Person<'a>>>) {
    let mut nodes: Vec<&str> = graph.keys().map(|name| name.to_owned()).collect();
    nodes.push("me");
    let mut permuations = permutohedron::Heap::new(&mut nodes);
    let mut max_distance = i32::MIN;
    while let Some(permu) = permuations.next_permutation() {
        let mut current_distance = 0;
        for idx in 0..permu.len() {
            let current = permu[idx];
            let next = permu[(idx + 1) % permu.len()];
            current_distance += if current == "me" || next == "me" {
                0
            } else {
                get_happiness(&graph, current, next)
            };
            current_distance += if current == "me" || next == "me" {
                0
            } else {
                get_happiness(&graph, next, current)
            };
        }
        // println!("{:?}", current_distance);
        max_distance = cmp::max(max_distance, current_distance);
    }
    println!("{max_distance}");
}

fn get_happiness<'a>(graph: &HashMap<&'a str, Vec<Person<'a>>>, current: &str, next: &str) -> i32 {
    let happy = graph[current].iter().find(|p| p.name == next).unwrap();

    happy.happiness
}

fn parse<'a>(line: &'a str, graph: &mut HashMap<&'a str, Vec<Person<'a>>>) {
    let context: Vec<_> = line.split(" ").collect();
    let from = context[0];
    let to = context[context.len() - 1].trim_matches('.');
    let did_lose = context[2];
    let amount: i32 = context[3].parse().unwrap();

    graph.entry(from).or_insert(Vec::new()).push(Person {
        name: to,
        happiness: if did_lose == "gain" { amount } else { -amount },
    });
}
