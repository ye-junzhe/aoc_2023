use std::{fs, collections::HashMap, ops::Deref};

#[derive(Default, Debug)]
struct DAY7 {
    data: String,
}

#[derive(Default, Debug, Clone, Copy)]
enum CardType {
    #[default] Unknow,
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Default, Debug)]
struct Card {
    hand: String,
    bid: u64,
    card_type: CardType,
    card_score: String,
}

#[derive(Default, Debug)]
struct Cards {
    inner: Vec<Card>,
}

#[derive(Default, Debug)]
struct CardDigitized {
    hand: String,
    bid: u64,
    card_type: CardType,
    card_score: u128,
}

#[derive(Default, Debug)]
struct CardsDigitized {
    inner: Vec<CardDigitized>,
}

const FILE_PATH: &str = "input.txt";

impl DAY7 {
    fn construct() -> Self {
        DAY7 {
            data: fs::read_to_string(FILE_PATH).expect("Should have been able to read the file"),
        }
    }

    fn to_win(&self) {
        let mut cards: Cards = Default::default();

        let mut total_winnings: u64 = 0;

        self.data.lines().for_each(|line| {
            let mut card = Card {
                hand: line.split_whitespace().nth(0).unwrap().to_string(),
                bid: line.split_whitespace().nth(1).unwrap().parse::<u64>().unwrap(),
                ..Default::default()
            };
            let (card_type, card_score) = card.determine_card_type();

            card.card_type = card_type;
            card.card_score = card_score;

            cards.inner.push(card);
        });

        dbg!(&cards);

        let mut cards_digitized = CardsDigitized {
            inner: {
                cards.inner.iter().map(|card| {
                    let digitized = card.card_score.parse::<u128>().unwrap();
                    CardDigitized {
                        hand: card.hand.clone(),
                        bid: card.bid,
                        card_type: card.card_type,
                        card_score: digitized,
                    }
                }).collect()
            }
        };

        cards_digitized.inner.sort_by_key(|card| card.card_score);
        dbg!(&cards_digitized);
        cards_digitized.inner.iter().enumerate().for_each(|(mut index, card)| {
            index += 1;
            // dbg!(index);
            // dbg!(card.bid);
            total_winnings += card.bid * index as u64;
        });

        dbg!(total_winnings);
    }
}

impl Card {
    fn determine_card_type(&mut self) -> (CardType, String) {
        let mut counter: HashMap<char, u64> = Default::default();
        let mut card_score: String = Default::default();

        // dbg!(&self.hand);
        let mut rev_hand = self.hand.chars().rev().collect::<String>();
        while let Some(c) = rev_hand.pop() {
            match c {
                'A' => {
                    counter.entry('A').and_modify(|counter| *counter += 1).or_insert(1);
                    card_score += "1110";
                }
                'K' => {
                    counter.entry('K').and_modify(|counter| *counter += 1).or_insert(1);
                    card_score += "1011";
                }
                'Q' => {
                    counter.entry('Q').and_modify(|counter| *counter += 1).or_insert(1);
                    card_score += "1010";
                }
                'J' => {
                    counter.entry('J').and_modify(|counter| *counter += 1).or_insert(1);
                    card_score += "1001";
                }
                'T' => {
                    counter.entry('T').and_modify(|counter| *counter += 1).or_insert(1);
                    card_score += "1000";
                }
                '9' => {
                    counter.entry('9').and_modify(|counter| *counter += 1).or_insert(1);
                    card_score += "0111";
                }
                '8' => {
                    counter.entry('8').and_modify(|counter| *counter += 1).or_insert(1);
                    card_score += "0110";
                }
                '7' => {
                    counter.entry('7').and_modify(|counter| *counter += 1).or_insert(1);
                    card_score += "0101";
                }
                '6' => {
                    counter.entry('6').and_modify(|counter| *counter += 1).or_insert(1);
                    card_score += "0100";
                }
                '5' => {
                    counter.entry('5').and_modify(|counter| *counter += 1).or_insert(1);
                    card_score += "0011";
                }
                '4' => {
                    counter.entry('4').and_modify(|counter| *counter += 1).or_insert(1);
                    card_score += "0010";
                }
                '3' => {
                    counter.entry('3').and_modify(|counter| *counter += 1).or_insert(1);
                    card_score += "0001";
                }
                '2' => {
                    counter.entry('2').and_modify(|counter| *counter += 1).or_insert(1);
                    card_score += "0000";
                }
                _ => panic!("Can not find the lable in the HashMap counter"),
            };
        };
        // dbg!(&counter);
        // 32T3K
        // KTJJT
        // KK677
        // T55J5
        // QQQJA

        let mut values: Vec<u64> = Default::default();
        for v in counter.values() {
            values.push(*v);
        }
        values.sort();
        // dbg!(&values);
        let mut s: String = Default::default();
        s.extend(values.iter().map(|&x| x.to_string()));
        let card_type = match s.deref() {
            "5" => {
                card_score = "1111".to_string() + &card_score;
                CardType::FiveOfAKind
            }
            "14" => {
                card_score = "1110".to_string() + &card_score;
                CardType::FourOfAKind
            }
            "23" => {
                card_score = "1101".to_string() + &card_score;
                CardType::FullHouse
            }
            "113" => {
                card_score = "1100".to_string() + &card_score;
                CardType::ThreeOfAKind
            }
            "122" => {
                card_score = "1011".to_string() + &card_score;
                CardType::TwoPair
            }
            "1112" => {
                card_score = "1010".to_string() + &card_score;
                CardType::OnePair
            }
            "11111" => {
                card_score = "1001".to_string() + &card_score;
                CardType::HighCard
            }
            _ => {
                CardType::Unknow
            }
        };

        (card_type, card_score)
    }

    // fn determine_card_type2(&mut self) -> (CardType, String) {
    //     let mut counter: HashMap<char, u64> = Default::default();
    //     let mut card_score: String = Default::default();
    //
    //     // dbg!(&self.hand);
    //     let mut rev_hand = self.hand.chars().rev().collect::<String>();
    //     while let Some(c) = rev_hand.pop() {
    //         match c {
    //             'A' => {
    //                 counter.entry('A').and_modify(|counter| *counter += 1).or_insert(1);
    //                 card_score += "1110";
    //             }
    //             'K' => {
    //                 counter.entry('K').and_modify(|counter| *counter += 1).or_insert(1);
    //                 card_score += "1011";
    //             }
    //             'Q' => {
    //                 counter.entry('Q').and_modify(|counter| *counter += 1).or_insert(1);
    //                 card_score += "1010";
    //             }
    //             'J' => {
    //                 counter.entry('J').and_modify(|counter| *counter += 1).or_insert(1);
    //                 card_score += "JJJJ";
    //             }
    //             'T' => {
    //                 counter.entry('T').and_modify(|counter| *counter += 1).or_insert(1);
    //                 card_score += "1000";
    //             }
    //             '9' => {
    //                 counter.entry('9').and_modify(|counter| *counter += 1).or_insert(1);
    //                 card_score += "0111";
    //             }
    //             '8' => {
    //                 counter.entry('8').and_modify(|counter| *counter += 1).or_insert(1);
    //                 card_score += "0110";
    //             }
    //             '7' => {
    //                 counter.entry('7').and_modify(|counter| *counter += 1).or_insert(1);
    //                 card_score += "0101";
    //             }
    //             '6' => {
    //                 counter.entry('6').and_modify(|counter| *counter += 1).or_insert(1);
    //                 card_score += "0100";
    //             }
    //             '5' => {
    //                 counter.entry('5').and_modify(|counter| *counter += 1).or_insert(1);
    //                 card_score += "0011";
    //             }
    //             '4' => {
    //                 counter.entry('4').and_modify(|counter| *counter += 1).or_insert(1);
    //                 card_score += "0010";
    //             }
    //             '3' => {
    //                 counter.entry('3').and_modify(|counter| *counter += 1).or_insert(1);
    //                 card_score += "0001";
    //             }
    //             '2' => {
    //                 counter.entry('2').and_modify(|counter| *counter += 1).or_insert(1);
    //                 card_score += "0000";
    //             }
    //             _ => panic!("Can not find the lable in the HashMap counter"),
    //         };
    //     };
    //
    //     let most_counted = &counter
    //         .iter()
    //         .max_by(|a, b| a.1.cmp(&b.1))
    //         .map(|(k, _v)| k)
    //         .unwrap();
    //     let mut if_is_j: u64 = 0;
    //     if *most_counted == &'J' ||
    //     ( counter.get(*most_counted) == Some(&2) && counter.get(&'J') == Some(&2) ) {
    //         if_is_j = *counter.get(&'J').unwrap();
    //         counter.remove(&'J');
    //     } else if *most_counted == &'J' && *counter.get(&'J').unwrap() == 1 {
    //         counter.entry('A').and_modify(|v| *v += 1).or_insert(0);
    //     }
    //     let cloned = counter.clone();
    //     let most_counted = cloned
    //         .iter()
    //         .max_by(|a, b| a.1.cmp(&b.1))
    //         .map(|(k, _v)| k)
    //         .unwrap();
    //     counter.entry(*most_counted).and_modify(|v| *v += if_is_j);
    //     // dbg!(most_counted);
    //
    //     match most_counted {
    //         'A' => {
    //             card_score = card_score.replacen("JJJJ", "1110", 6);
    //             if *counter.get(most_counted).unwrap() == 1 {
    //                 self.hand = self.hand.replace("J", "A");
    //             } else {
    //                 self.hand = self.hand.replacen("J", "A", 5)
    //             }
    //         }
    //         'K' => {
    //             card_score = card_score.replacen("JJJJ", "1011", 6);
    //             if *counter.get(most_counted).unwrap() == 1 {
    //                 self.hand = self.hand.replace("J", "A");
    //             } else {
    //                 self.hand = self.hand.replacen("J", "K", 5)
    //             }
    //         }
    //         'Q' => {
    //             card_score = card_score.replacen("JJJJ", "1010", 6);
    //             if *counter.get(most_counted).unwrap() == 1 {
    //                 self.hand = self.hand.replace("J", "A");
    //             } else {
    //                 self.hand = self.hand.replacen("J", "Q", 5)
    //             }
    //         }
    //         'J' => {
    //             card_score = card_score.replacen("JJJJ", "1001", 6);
    //             self.hand = self.hand.replace("J", "A");
    //         }
    //         'T' => {
    //             card_score = card_score.replacen("JJJJ", "1000", 6);
    //             if *counter.get(most_counted).unwrap() == 1 {
    //                 self.hand = self.hand.replace("J", "A");
    //             } else {
    //                 self.hand = self.hand.replacen("J", "T", 5)
    //             }
    //         }
    //         '9' => {
    //             card_score = card_score.replacen("JJJJ", "0111", 6);
    //             if *counter.get(most_counted).unwrap() == 1 {
    //                 self.hand = self.hand.replace("J", "A");
    //             } else {
    //                 self.hand = self.hand.replacen("J", "9", 5)
    //             }
    //         }
    //         '8' => {
    //             card_score = card_score.replacen("JJJJ", "0110", 6);
    //             if *counter.get(most_counted).unwrap() == 1 {
    //                 self.hand = self.hand.replace("J", "A");
    //             } else {
    //                 self.hand = self.hand.replacen("J", "8", 5)
    //             }
    //         }
    //         '7' => {
    //             card_score = card_score.replacen("JJJJ", "0101", 6);
    //             if *counter.get(most_counted).unwrap() == 1 {
    //                 self.hand = self.hand.replace("J", "A");
    //             } else {
    //                 self.hand = self.hand.replacen("J", "7", 5)
    //             }
    //         }
    //         '6' => {
    //             card_score = card_score.replacen("JJJJ", "0100", 6);
    //             if *counter.get(most_counted).unwrap() == 1 {
    //                 self.hand = self.hand.replace("J", "A");
    //             } else {
    //                 self.hand = self.hand.replacen("J", "6", 5)
    //             }
    //         }
    //         '5' => {
    //             card_score = card_score.replacen("JJJJ", "0011", 6);
    //             if *counter.get(most_counted).unwrap() == 1 {
    //                 self.hand = self.hand.replace("J", "A");
    //             } else {
    //                 self.hand = self.hand.replacen("5", "J", 5)
    //             }
    //         }
    //         '4' => {
    //             card_score = card_score.replacen("JJJJ", "0010", 6);
    //             if *counter.get(most_counted).unwrap() == 1 {
    //                 self.hand = self.hand.replace("J", "A");
    //             } else {
    //                 self.hand = self.hand.replacen("4", "J", 5)
    //             }
    //         }
    //         '3' => {
    //             card_score = card_score.replacen("JJJJ", "0001", 6);
    //             if *counter.get(most_counted).unwrap() == 1 {
    //                 self.hand = self.hand.replace("J", "A");
    //             } else {
    //                 self.hand = self.hand.replacen("3", "J", 5)
    //             }
    //         }
    //         '2' => {
    //             card_score = card_score.replacen("JJJJ", "0000", 6);
    //             if *counter.get(most_counted).unwrap() == 1 {
    //                 self.hand = self.hand.replace("J", "A");
    //             } else {
    //                 self.hand = self.hand.replacen("2", "J", 5)
    //             }
    //         }
    //         _ => panic!("Can not find the lable in the HashMap counter"),
    //     }
    //     // dbg!(&card_score);
    //     // dbg!(&counter);
    //
    //     // WARNING: NEW ORDER
    //     // 32T3K
    //     // KK677
    //     // T55J5
    //     // QQQJA
    //     // KTJJT
    //
    //     let mut values: Vec<u64> = Default::default();
    //     for v in counter.values() {
    //         values.push(*v);
    //     }
    //     values.sort();
    //     dbg!(&values);
    //     let mut s: String = Default::default();
    //     s.extend(values.iter().map(|&x| x.to_string()));
    //     let card_type: CardType = Default::default();
    //     println!("{}", s.deref());
    //     match s.deref() {
    //         "5" => {
    //             card_score = "1111".to_string() + &card_score;
    //             CardType::FiveOfAKind
    //         }
    //         "14" => {
    //             card_score = "1110".to_string() + &card_score;
    //             CardType::FourOfAKind
    //         }
    //         "23" => {
    //             card_score = "1101".to_string() + &card_score;
    //             CardType::FullHouse
    //         }
    //         "113" => {
    //             card_score = "1100".to_string() + &card_score;
    //             CardType::ThreeOfAKind
    //         }
    //         "122" => {
    //             card_score = "1011".to_string() + &card_score;
    //             CardType::TwoPair
    //         }
    //         "1112" => {
    //             card_score = "1010".to_string() + &card_score;
    //             CardType::OnePair
    //         }
    //         "11111" => {
    //             card_score = "1001".to_string() + &card_score;
    //             CardType::HighCard
    //         }
    //         _ => {
    //             CardType::Unknow
    //         }
    //     };
    //
    //     (card_type, card_score)
    // }
}

fn main() {
    let day_7 = DAY7::construct();
    day_7.to_win();
}
