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
        if key.ends_with("A") {
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

fn lcm(numbers: Vec<usize>) -> usize {
    let mut result = numbers[0];
    for i in 1..numbers.len() {
        result = (numbers[i] * result) / gcd(numbers[i], result);
    }
    result
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 { return a; }
    gcd(b, a % b)
}

fn part_1(filename: &str) -> i32 {
    let network = setup(filename);
    let mut steps = 0;
    let mut current = "AAA".to_string();
    let mut i = 0;
    while current != *"ZZZ" {
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

fn part_2(filename: &str) -> usize {
    let network = setup(filename);
    let mut loops = Vec::new();

    for starter in network.starters {
        let mut steps = 0;
        let mut current = starter;
        let mut i = 0;
        while !current.ends_with("Z") {
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
        loops.push(steps as usize);
    }
    lcm(loops)
}

fn main() {
    println!("{}", part_1("example.txt"));
    println!("{}", part_1("input.txt"));
    println!("{}", part_2("example_2.txt"));
    println!("{}", part_2("input.txt"));
}
