use permutohedron;
use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::fs;

// fix using permut library
// while let Some(permutation) = permutations.next_permutation() {

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    happiness: i32,
}

#[derive(Debug)]
struct h_info<'a> {
    node: &'a str,
    happiness: i32,
    previous: Option<&'a str>,
    visitied: HashSet<&'a str>,
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

fn find_happiness(graph: &HashMap<&str, Vec<Person>>) {
    let start = "Alice";

    let mut stack = Vec::from([h_info {
        node: start,
        happiness: 0,
        previous: None,
        visitied: HashSet::new(),
    }]);

    let mut max_happiness = i32::MIN;
    while stack.len() > 0 {
        let mut current_node = stack.pop().unwrap();

        current_node.visitied.insert(current_node.node);

        if !current_node.previous.is_none() {
            let person = &graph[current_node.node]
                .iter()
                .find(|p| p.name == current_node.previous.unwrap())
                .unwrap();
            current_node.happiness += person.happiness;
        }

        println!("{current_node:?}");
        if current_node.visitied.len() == graph.keys().len() {
            // last node
            let end = &graph[current_node.node]
                .iter()
                .find(|p| p.name == start)
                .unwrap();
            let start = &graph[start]
                .iter()
                .find(|p| p.name == current_node.node)
                .unwrap();
            current_node.happiness += end.happiness + start.happiness;
            max_happiness = max(max_happiness, current_node.happiness);
            println!("{max_happiness}");
            continue;
        }

        for person in &graph[current_node.node] {
            if current_node.visitied.contains(person.name) {
                continue;
            }

            stack.push(h_info {
                node: person.name,
                happiness: current_node.happiness + person.happiness,
                previous: Some(current_node.node),
                visitied: current_node.visitied.clone(),
            })
        }
    }
}

pub fn run_code() {
    let path = "q13.txt";
    let context = fs::read_to_string(path).expect("expect to read a file");

    let mut graph = HashMap::new();

    context.lines().for_each(|line| parse(line, &mut graph));
    println!("{:?}", graph);

    find_happiness(&graph);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
