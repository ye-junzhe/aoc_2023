use std::fs;

#[derive(Default, Debug)]
struct DAY9 {
    data: String,
}

const FILE_PATH: &str = "input.txt";

impl DAY9 {
    fn construct() -> Self {
        DAY9 {
            data: fs::read_to_string(FILE_PATH).expect("Should have been able to read the file"),
        }
    }

    fn sum_p1(&self) {
        let mut result: i64 = Default::default();
        self.data.lines().for_each(|line| {
            // dbg!(&line);
            let mut nums = line.split(" ").map(|char| char.parse::<i64>().unwrap()).collect::<Vec<i64>>();
            let mut lines_of_nums: Vec<Vec<i64>> = Default::default();
            lines_of_nums.push(nums.clone());
            loop {
                let mut new_nums: Vec<i64> = Default::default();
                let zeros = nums.iter().filter(|n| **n == 0).count();
                if zeros == nums.iter().count() {
                    let counter = lines_of_nums.iter().count();
                    let mut new_last = 0;
                    for i in (0..counter).rev() {
                        new_last += lines_of_nums[i].iter().last().unwrap();
                    }
                    result += new_last;
                    break;
                }
                for (ind, _num) in nums.iter().enumerate() {
                    if ind > 0 {
                        let new_num = nums[ind] - nums[ind - 1];
                        new_nums.push(new_num);
                    }
                }
                lines_of_nums.push(new_nums.clone());
                nums = new_nums;
            }
        });
        dbg!(result);
    }

    fn sum(&self) {
        let mut result: Vec<i64> = Default::default();
        self.data.lines().for_each(|line| {
            // dbg!(&line);
            let mut nums = line.split(" ").map(|char| char.parse::<i64>().unwrap()).collect::<Vec<i64>>();
            let mut lines_of_nums: Vec<Vec<i64>> = Default::default();
            lines_of_nums.push(nums.clone());
            loop {
                let mut new_nums: Vec<i64> = Default::default();
                let zeros = nums.iter().filter(|n| **n == 0).count();
                // if zeros == nums.iter().count() {
                //     let counter = lines_of_nums.iter().count();
                //     let mut new_last = 0;
                //     for i in (0..counter).rev() {
                //         new_last += lines_of_nums[i].iter().last().unwrap();
                //     }
                //     result += new_last;
                //     break;
                // }
                if zeros == nums.iter().count() {
                    let counter = lines_of_nums.iter().count();
                    let mut first_line_new_first: i64 = Default::default();
                    for i in (1..counter).rev() {
                        let new_first = lines_of_nums[i-1][0] - lines_of_nums[i][0];
                        lines_of_nums[i-1].insert(0, new_first);
                        // dbg!(new_first);
                        first_line_new_first = new_first;
                    }
                    result.push(first_line_new_first);
                    break;
                }
                for (ind, _num) in nums.iter().enumerate() {
                    if ind > 0 {
                        let new_num = nums[ind] - nums[ind - 1];
                        new_nums.push(new_num);
                    }
                }
                lines_of_nums.push(new_nums.clone());
                nums = new_nums;
            }
        });
        dbg!(result.iter().sum::<i64>());
        // dbg!(result);
    }
}
fn main() {
    let day_9 = DAY9::construct();
    day_9.sum_p1();
    day_9.sum();
}
