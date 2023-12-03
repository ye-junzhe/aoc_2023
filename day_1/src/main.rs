use std::{fs, process};

#[derive(Default)]
struct DAY1 {
    data: String,
}

const FILE_PATH1: &str = "input1.txt";
const FILE_PATH2: &str = "input2.txt";

impl DAY1 {
    fn part_1() -> Self {
        let day_1 = DAY1 {
            data: fs::read_to_string(FILE_PATH1).expect("Should have been able to read the file"),
        };
        day_1
    }

    fn part_2() -> Self {
        let day_1 = DAY1 {
            data: fs::read_to_string(FILE_PATH2).expect("Should have been able to read the file"),
        };
        day_1
    }

    fn get_number(&self) {
        let mut total: u32 = u32::default();
        let _ = self.data.lines().for_each(|line| {
            let numbers: Vec<_> = line.chars().filter_map(|char| {
                if char.is_numeric() {
                    Some(char.to_digit(10).unwrap()) // Convert char to u32
                } else {
                    None
                }
            }).collect();

            let mut result: u32 = u32::default();
            let first = numbers[0];
            let last = numbers.iter().last().unwrap();
            result = first * 10 + last;
            // println!("Numbers in line: {:?}", numbers);
            // println!("Result of this line is: {}", result);
            total += result;
        });
        dbg!(total);
    }

    fn process(&self) {
        let output = self.data.lines().map(process_line).sum::<u32>();
        dbg!(output);
    }

}

fn process_line(line: &str) -> u32 {
    let mut it = (0..line.len()).filter_map(|index| {
        let reduced_line = &line[index..];
        let result = if reduced_line.starts_with("one") {
            '1'
        } else if reduced_line.starts_with("two") {
            '2'
        } else if reduced_line.starts_with("three") {
            '3'
        } else if reduced_line.starts_with("four") {
            '4'
        } else if reduced_line.starts_with("five") {
            '5'
        } else if reduced_line.starts_with("six") {
            '6'
        } else if reduced_line.starts_with("seven") {
            '7'
        } else if reduced_line.starts_with("eight") {
            '8'
        } else if reduced_line.starts_with("nine") {
            '9'
        } else {
            reduced_line.chars().next().unwrap()
        };

        result.to_digit(10)
    });
    let first = it.next().expect("should be a number");

    match it.last() {
        Some(num) => format!("{first}{num}"),
        None => format!("{first}{first}"),
    }
        .parse::<u32>()
        .expect("should be a valid number")
}

fn main() {
    println!("Hello, this is Day 1 !");
    let day_1_part1 = DAY1::part_1();
    let day_1_part2 = DAY1::part_2();
    day_1_part1.get_number();
    day_1_part2.process();
}
