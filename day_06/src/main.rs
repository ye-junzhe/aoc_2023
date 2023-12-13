use std::fs;

#[derive(Default, Debug)]
struct DAY6 {
    data: String,
}

const FILE_PATH: &str = "input.txt";

impl DAY6 {
    fn construct() -> Self {
        DAY6 {
            data: fs::read_to_string(FILE_PATH).expect("Should have been able to read the file"),
        }
    }

    fn to_win(&self) {
        let mut times: Vec<u64> = Default::default();
        let mut distances: Vec<u64> = Default::default();
        self.data.lines().enumerate().for_each(|(line_number, line_data)| {
            if line_number == 0 {
                times = line_data
                    .split_whitespace()
                    .collect::<Vec<&str>>()
                    .iter().filter_map(|x| x.parse::<u64>().ok()).collect::<Vec<u64>>();
                // dbg!(&times);
            }
            if line_number == 1 {
                distances = line_data
                    .split_whitespace()
                    .collect::<Vec<&str>>()
                    .iter().filter_map(|x| x.parse::<u64>().ok()).collect::<Vec<u64>>();
                // dbg!(&distances);
            }
        });

        let mut times_part2: Vec<u64> = Default::default();
        let mut distances_part2: Vec<u64> = Default::default();
        self.data.lines().enumerate().for_each(|(line_number, line_data)| {
            if line_number == 0 {
                times_part2 = line_data
                    .split(":")
                    .nth(1)
                    .unwrap()
                    .split_whitespace()
                    .collect::<Vec<&str>>()
                    .iter().filter_map(|x| x.parse::<u64>().ok()).collect::<Vec<u64>>();
                // dbg!(&times);
            }
            if line_number == 1 {
                distances_part2 = line_data
                    .split(":")
                    .nth(1)
                    .unwrap()
                    .split_whitespace()
                    .collect::<Vec<&str>>()
                    .iter().filter_map(|x| x.parse::<u64>().ok()).collect::<Vec<u64>>();
                // dbg!(&distances);
            }
        });
        let mut real_times_part2: String = Default::default();
        for num in times_part2 {
            let string = num.to_string();
            for char in string.chars() {
                real_times_part2.push(char);
            }
        }
        let real_times = real_times_part2.parse::<u64>().unwrap();
        dbg!(real_times);

        let mut real_distances_part2: String = Default::default();
        for num in &distances_part2 {
            let string = num.to_string();
            for char in string.chars() {
                real_distances_part2.push(char);
            }
        }
        let real_dist = real_distances_part2.parse::<u64>().unwrap();
        dbg!(real_dist);

        let mut count = 0;
        let mut part1_result = 1;
        times.iter().for_each(|time| {
            let mut win_times = 0;
            let press_time = 1..=*time;
            press_time.into_iter().for_each(|p_time| {
                let milimeters = p_time * (time - p_time);
                if milimeters > distances[count] {
                    win_times += 1;
                }
            });
            count += 1;
            part1_result *= win_times;
            // dbg!(win_times);
        });
        dbg!(part1_result);

        let mut part2_result = 0;
        let mut win_times = 0;
        let press_time = 1..=real_times;
        press_time.into_iter().for_each(|p_time| {
            let milimeters = p_time * (real_times - p_time);
            if milimeters > real_dist {
                win_times += 1;
            }
        });
        part2_result += win_times;
        dbg!(part2_result);

    }
}

fn main() {
    let day_6 = DAY6::construct();
    day_6.to_win();
}
