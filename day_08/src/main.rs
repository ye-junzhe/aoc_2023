use std::{fs, u128};

#[derive(Default, Debug)]
struct DAY8 {
    data: String,
}

#[derive(Default, Debug)]
struct Node {
    me: String,
    left: String,
    right: String,
}

#[derive(Default, Debug)]
struct Nodes {
    inner: Vec<Node>,
}

const FILE_PATH: &str = "input.txt";

impl DAY8 {
    fn construct() -> Self {
        DAY8 {
            data: fs::read_to_string(FILE_PATH).expect("Should have been able to read the file"),
        }
    }

    fn to_find(&self) {
        let mut splits = self.data.split("\n\n");
        let instructions = splits.next().unwrap();
        let split_latter = splits.next().unwrap();
        let mut nodes: Nodes = Default::default();
        split_latter.lines().for_each(|line| {
            // dbg!(line);
            let node: Vec<_> = line.chars().filter(|x| x.is_alphabetic()).collect();
            nodes.inner.push( Node {
                me: String::from_iter(node[0..3].iter()),
                left: String::from_iter(node[3..6].iter()),
                right: String::from_iter(node[6..9].iter()),
            })
        });
        // dbg!(&nodes);
        let mut pointer = 0;
        let mut counter_part1 = 0;
        for c in instructions.chars().cycle() {
            if &nodes.inner[pointer].me == "ZZZ" {
                break;
            }
            match c {
                'L' => {
                    let goto = &nodes.inner[pointer].left;
                    let goto_index = nodes.inner.iter().enumerate().filter(|(index, node)| &node.me == goto).next().unwrap();
                    pointer = goto_index.0;
                    // dbg!(&nodes.inner[pointer]);
                    counter_part1 += 1;
                }
                'R' => {
                    let goto = &nodes.inner[pointer].right;
                    let goto_index = nodes.inner.iter().enumerate().filter(|(index, node)| &node.me == goto).next().unwrap();
                    pointer = goto_index.0;
                    // dbg!(&nodes.inner[pointer]);
                    counter_part1 += 1;
                }
                _ => (),
            }
        }
        dbg!(counter_part1);

        let mut splits = self.data.split("\n\n");
        let instructions = splits.next().unwrap();
        let split_latter = splits.next().unwrap();
        let mut nodes: Nodes = Default::default();
        split_latter.lines().for_each(|line| {
            // dbg!(line);
            let node: Vec<_> = line.chars().collect();
            nodes.inner.push( Node {
                me: String::from_iter(node[0..3].iter()),
                left: String::from_iter(node[7..10].iter()),
                right: String::from_iter(node[12..15].iter()),
            })
        });

        let mut starting_pointers: Vec<(usize, &Node)> = nodes.inner.iter().enumerate().filter(|(_, node)| node.me.chars().last().unwrap() == 'A').collect::<Vec<(usize, &Node)>>();
        // dbg!(&starting_pointers);
        let mut counter = 0;
        for index in 0..starting_pointers.iter().count() {
            let initial = &starting_pointers[index].clone();
            let mut final_counts: Vec<u128> = Default::default();
            for c in instructions.chars().cycle() {
                if starting_pointers[index].1.me.chars().last().unwrap() == 'Z' {
                    final_counts.push(counter);
                    println!("{} -> {}: {}", initial.1.me, &starting_pointers[index].1.me, counter);
                    break;
                } else {
                    counter += 1;
                }
                match c {
                    'L' => {
                        let goto = &starting_pointers[index].1.left;
                        let goto_index = nodes.inner.iter().enumerate().filter(|(_, node)| &node.me == goto).next().unwrap();
                        starting_pointers[index] = goto_index;
                        // dbg!(&starting_pointers);
                    }
                    'R' => {
                        let goto = &starting_pointers[index].1.right;
                        let goto_index = nodes.inner.iter().enumerate().filter(|(_, node)| &node.me == goto).next().unwrap();
                        starting_pointers[index] = goto_index;
                        // dbg!(&starting_pointers);
                    }
                    _ => (),
                }
            }
        }
        println!("Not finished yet, calculate the LCM of those numbers online to get the result of part2");
    }
}

fn main() {
    let day_8 = DAY8::construct();
    day_8.to_find();
    println!("Hello, world!");
}
