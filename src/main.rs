use std::{fs::read_to_string, collections::HashMap};

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

#[derive(Debug)]
struct Network {
    instructions: Vec<char>,
    nodes: HashMap<String, Node>,
    starters: Vec<String>,
}

fn setup(filename: &str) -> Network {
    let contents = read_to_string(filename)
        .expect("Something went wrong reading the file");
    let mut lines = contents.lines();
    let mut nodes = HashMap::new();

    let instructions = lines.next().unwrap().to_string().chars().collect::<Vec<char>>();
    lines.next();
    let mut starters = Vec::new();
    for line in lines {
        let mut parts = line.split("=");
        let key = parts.next().unwrap().trim();
        if key.chars().nth(2).unwrap() == 'A' {
            starters.push(key.to_string());
        } 
        let value = parts.next().unwrap().trim();
        let mut parts = value.split(",");
        let left = parts.next().unwrap().trim().chars().filter(|c| c.is_alphanumeric()).collect();
        let right = parts.next().unwrap().trim().chars().filter(|c| c.is_alphanumeric()).collect();

        nodes.insert(key.to_string(), Node { left, right });
    };
    Network { instructions, nodes, starters }
}

fn part_1(filename: &str) -> i32 {
    let network = setup(filename);
    let mut steps = 0;
    let mut current = "AAA".to_string();
    let mut i = 0;
    while current != String::from("ZZZ") {
        let node = network.nodes.get(&current).unwrap();
        let instruction = network.instructions[i];
        if instruction == 'L' {
            current = node.left.clone();
        } else {
            current = node.right.clone();
        }
        steps += 1;
        i += 1;
        if i == network.instructions.len() { i = 0; }
    }
    steps
}

fn part_2(filename: &str) -> i32 {
    let network = setup(filename);
    let mut steps = 0;
    let mut starters = network.starters;
    let mut i = 0;

    println!("{:?}", starters);
    while starters.iter().any(|s| s.chars().nth(2).unwrap() != 'Z') {
        let mut new_starters = Vec::new();
        let instruction = network.instructions[i];
        for current in starters {
            let node = network.nodes.get(&current).unwrap();
            if instruction == 'L' {
                new_starters.push(node.left.clone());
            } else {
                new_starters.push(node.right.clone());
            }
        }
        starters = new_starters;
        steps += 1;
        i += 1;
        if i == network.instructions.len() { i = 0; }
        // println!("{:?}", starters);
    }
    steps
}

fn main() {
    assert_eq!(part_1("example.txt"), 6);
    assert_eq!(part_1("input.txt"), 15989);
    assert_eq!(part_2("example_2.txt"), 6);
    println!("part 1: {}", part_2("input.txt"));
    //assert_eq!(part_2("input.txt"), 0);
}
