use std::fs;

#[derive(Default)]
struct DAY2 {
    data: String,
}

const FILE_PATH1: &str = "input1.txt";
const FILE_PATH2: &str = "input2.txt";

impl DAY2 {
    fn part_1() -> Self {
        DAY2 {
            data: fs::read_to_string(FILE_PATH1).expect("Should have been able to read the file"),
        }
    }

    fn part_2() -> Self {
        DAY2 {
            data: fs::read_to_string(FILE_PATH2).expect("Should have been able to read the file"),
        }
    }

    fn sum_ids(&self) {
        let mut sum: usize = Default::default();
        let mut game_round: usize = Default::default();
        self.data.lines().enumerate().for_each(|(index, line)| {

            let mut if_less_than_39: bool = true;
            let mut red_cubes: u32 = Default::default();
            let mut green_cubes: u32 = Default::default();
            let mut blue_cubes: u32 = Default::default();

            game_round = (index + 1).into();

            let all_sets = line.split(": ").collect::<Vec<_>>()[1];
            let split_sets = all_sets.split("; ").collect::<Vec<_>>();
            split_sets.iter().for_each(|set| {
                set.split(", ").collect::<Vec<_>>().iter().for_each(|num_and_color| {
                    // dbg!(color.split(" ").collect::<Vec<_>>());
                    let single_cube_type = num_and_color.split(" ").collect::<Vec<_>>();
                    let num: u32 = single_cube_type[0].parse::<u32>().unwrap();
                    let color = single_cube_type[1];

                    match color {
                        "red" => red_cubes = std::cmp::max(num, red_cubes),
                        "green" => green_cubes = std::cmp::max(num, green_cubes),
                        "blue" => blue_cubes = std::cmp::max(num, blue_cubes),
                        _ => println!("Not a color"),
                    }
                    
                    if red_cubes + green_cubes + blue_cubes >= 39 {
                        if_less_than_39 = false;
                    }
                });
            });
            // println!("========================");
            // dbg!(game_round);
            if red_cubes <= 12 &&
            green_cubes <= 13 &&
            blue_cubes <= 14 &&
            if_less_than_39 {
                sum += game_round;
            }
            // dbg!(red_cubes);
            // dbg!(green_cubes);
            // dbg!(blue_cubes);
        });
        dbg!(sum);
    }

    fn sum_powers(&self) {
        let mut sum:u32 = Default::default();
        let mut one_round:u32 = Default::default();
        let mut game_round: usize = Default::default();
        self.data.lines().enumerate().for_each(|(index, line)| {

            let mut red_cubes: u32 = Default::default();
            let mut green_cubes: u32 = Default::default();
            let mut blue_cubes: u32 = Default::default();

            game_round = (index + 1).into();

            let all_sets = line.split(": ").collect::<Vec<_>>()[1];
            let split_sets = all_sets.split("; ").collect::<Vec<_>>();
            split_sets.iter().for_each(|set| {
                set.split(", ").collect::<Vec<_>>().iter().for_each(|num_and_color| {
                    // dbg!(color.split(" ").collect::<Vec<_>>());
                    let single_cube_type = num_and_color.split(" ").collect::<Vec<_>>();
                    let num: u32 = single_cube_type[0].parse::<u32>().unwrap();
                    let color = single_cube_type[1];

                    match color {
                        "red" => red_cubes = std::cmp::max(num, red_cubes),
                        "green" => green_cubes = std::cmp::max(num, green_cubes),
                        "blue" => blue_cubes = std::cmp::max(num, blue_cubes),
                        _ => println!("Not a color"),
                    }

                    
                });
            });
            // println!("========================");
            // dbg!(game_round);
            // dbg!(red_cubes);
            // dbg!(green_cubes);
            // dbg!(blue_cubes);
            one_round = red_cubes * green_cubes * blue_cubes;
            sum += one_round;
            // dbg!(one_round);
        });
        dbg!(sum);
    }
}

fn main() {
    println!("Hello, this is Day 2 !");
    let day_2_part1 = DAY2::part_1();
    day_2_part1.sum_ids();
    let day_2_part2 = DAY2::part_2();
    day_2_part2.sum_powers();
}
