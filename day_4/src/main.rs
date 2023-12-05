use std::{fs, collections::HashSet};

#[derive(Default, Debug)]
struct DAY4 {
    data: String,
}

const FILE_PATH: &str = "input.txt";

#[derive(Default, Debug, Clone)]
struct Card {
    card_type: u32,
    cards_to_win: u32,
}

#[derive(Default, Debug)]
struct Cards {
    inner: Vec<Card>,
}

impl DAY4 {
    fn construct() -> Self {
        DAY4 {
            data: fs::read_to_string(FILE_PATH).expect("Should have been able to read the file"),
        }
    }

    fn cal_worth(&self) {
        let mut sum_part1: u32 = Default::default();

        let mut cards: Cards = Default::default();
        let mut card_index: u32 = 0;

        self.data.lines().for_each(|line| {
            card_index += 1;

            let winning_nums: HashSet<u32> = line.split(": ").nth(1).unwrap().split(" | ").nth(0).unwrap().split_whitespace().map(|num| {
                num.parse::<u32>().unwrap()
            }).collect();
            let what_i_have: HashSet<u32> = line.split(": ").nth(1).unwrap().split(" | ").nth(1).unwrap().split_whitespace().map(|num| {
                num.parse::<u32>().unwrap()
            }).collect();

            // dbg!(winning_nums);
            // dbg!(what_i_have);

            let won: Vec<_> = winning_nums.intersection(&what_i_have).collect();
            let numbers_of_cards_to_get = won.iter().count();

            cards.inner.push(Card {
                card_type: card_index,
                cards_to_win: u32::try_from(numbers_of_cards_to_get).unwrap_or_default()
            });

            if !won.is_empty() {
                sum_part1 += 2u32.pow(u32::try_from(won
                    .iter()
                    .count()
                )
                    .unwrap()
                    .checked_sub(1)
                    .unwrap_or_default()
                );
            }
        });

        let mut total_num: usize;
        let mut count = 0usize;
        loop {
            // dbg!(count);
            for i in cards.inner[count].card_type..cards.inner[count].card_type + cards.inner[count].cards_to_win {
                cards.inner.push(cards.inner[i as usize].clone())
            }
            total_num = cards.inner.iter().count();
            count += 1;
            if count == total_num {
                break;
            }
        }

        let sum_part2 = cards.inner.iter().count();

        dbg!(sum_part1);
        dbg!(sum_part2);
    }
}

fn main() {
    println!("Hello, world!");
    let day_4 = DAY4::construct();
    day_4.cal_worth();
}
