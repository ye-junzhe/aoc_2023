use std::{fs, ops::Mul};

#[derive(Default, Debug)]
struct DAY3 {
    data: String,
}

const FILE_PATH1: &str = "input1.txt";
const FILE_PATH2: &str = "input2.txt";

#[derive(Default, Debug)]
struct Number {
    actual_num: String,
    num_coords: Vec<(usize, usize)>,
    is_added: bool,
}

#[derive(Default, Debug)]
struct Symbol {
    symbol: String,
    single_coords: (usize, usize),
    adjacent_to_num: Vec<String>,
    adjacent_to_count: usize,
}

#[derive(Default, Debug)]
struct Numbers {
    nums: Vec<Number>,
}

#[derive(Default, Debug)]
struct Symbols {
    syms: Vec<Symbol>,
}

impl DAY3 {
    fn part_1() -> Self {
        DAY3 {
            data: fs::read_to_string(FILE_PATH1).expect("Should have been able to read the file"),
        }
    }

    fn part_2() -> Self {
        DAY3 {
            data: fs::read_to_string(FILE_PATH2).expect("Should have been able to read the file"),
        }
    }

    fn find_parts(&self, is_part1: bool) {
        let mut numbers: Numbers = Numbers { nums: Vec::new() };
        let mut symbols: Symbols = Symbols { syms: Vec::new() };
        self.data.lines().enumerate().for_each(|(y, line)| {
            let mut digits: String = Default::default();
            let mut numbers_coords: Vec<(usize, usize)> = Vec::new();

            line.chars().enumerate().for_each(|(x, char)| {
                if char.is_digit(10) {
                    digits.push(char);
                    numbers_coords.push((y, x));
                    if x == line.len() - 1 && !digits.is_empty() {
                        numbers.nums.push(Number { actual_num: digits.clone(), num_coords: numbers_coords.clone(), is_added: false });
                    }
                } else {
                    match char {
                        '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => (),
                        '.' => (),
                        _ => symbols.syms.push(Symbol { symbol: char.to_string(), single_coords: (y, x), adjacent_to_num: Default::default(), adjacent_to_count: 0 }),
                    }
                    if !digits.is_empty() {
                        numbers.nums.push(Number { actual_num: digits.clone(), num_coords: numbers_coords.clone(), is_added: false });
                    }
                    digits.clear();
                    numbers_coords.clear();
                }
            });

            // dbg!(numbers_coords);
        });
        // dbg!(symbols);
        // dbg!(&numbers);

        numbers.find_adjacent_symbol(symbols, is_part1);
    }
}

impl Numbers {
    fn find_adjacent_symbol(&mut self, mut symbols: Symbols, is_part1: bool) {
        let mut adjacent_grids: Vec<(usize, usize)> = Vec::new();
        let mut sum_p1: usize = Default::default();
        let mut sum_p2: usize = Default::default();

        // *    *    *
        // *  (0,0)  *
        // *    *    *
        let _ = &self.nums.iter_mut().for_each(|number| {
            number.num_coords.iter().for_each(|coords| {
                adjacent_grids.push((coords.0, coords.1));
                adjacent_grids.push((coords.0, coords.1 + 1));
                adjacent_grids.push((coords.0, coords.1.checked_sub(1).unwrap_or_default()));
                adjacent_grids.push((coords.0 + 1, coords.1));
                adjacent_grids.push((coords.0 + 1, coords.1 + 1));
                adjacent_grids.push((coords.0 + 1, coords.1.checked_sub(1).unwrap_or_default()));
                adjacent_grids.push((coords.0.checked_sub(1).unwrap_or_default(), coords.1));
                adjacent_grids.push((coords.0.checked_sub(1).unwrap_or_default(), coords.1 + 1));
                adjacent_grids.push((coords.0.checked_sub(1).unwrap_or_default(), coords.1.checked_sub(1).unwrap_or_default()));
            });

            symbols.syms.iter_mut().for_each(|symbol| {
                if adjacent_grids.contains(&symbol.single_coords) {
                    if is_part1 {
                        if !number.is_added {
                            // println!("{}", number.actual_num);
                            sum_p1 += number.actual_num.parse::<usize>().unwrap_or_default();
                        }
                        number.is_added = true;
                    }
                    if !is_part1 {
                        symbol.adjacent_to_count += 1;
                        symbol.adjacent_to_num.push(number.actual_num.clone());
                        if symbol.adjacent_to_count == 2 {
                            // dbg!(&symbol);
                            let multi: usize = Mul::mul(symbol.adjacent_to_num[0].parse::<usize>().unwrap(), symbol.adjacent_to_num[1].parse::<usize>().unwrap());
                            sum_p2 += multi;
                        }
                    }
                };
            });

            adjacent_grids.clear();
        });

        if is_part1 { dbg!(sum_p1); }
        if !is_part1 { dbg!(sum_p2); }
    }
}


fn main() {
    println!("Hello, this is Day 2 !");
    let is_part1: bool = true;

    let day_3_part1 = DAY3::part_1();
    day_3_part1.find_parts(is_part1);

    let day_3_part2 = DAY3::part_2();
    day_3_part2.find_parts(!is_part1);
}
