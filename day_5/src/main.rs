use std::fs;

#[derive(Default, Debug)]
struct DAY5 {
    data: String,
}

const FILE_PATH: &str = "input.txt";

#[derive(Default, Debug, Clone)]
struct Map {
    dest: i64,
    source: i64,
    length: i64,
}

#[derive(Default, Debug, Clone)]
struct MapKind {
    kind: Vec<Map>,
}

#[derive(Default, Debug)]
struct Maps {
    map_kinds: Vec<MapKind>,
}

impl DAY5 {
    fn construct() -> Self {
        DAY5 {
            data: fs::read_to_string(FILE_PATH).expect("Should have been able to read the file"),
        }
    }

    fn map_to_location(&self) {
        let maps_and_seeds = self.data.split("\n\n").collect::<Vec<&str>>();
        let mut seeds = maps_and_seeds[0]
            .split(": ")
            .nth(1).unwrap()
            .split(" ")
            .map(|seed| {
                seed.parse::<i64>().unwrap()
            })
            .collect::<Vec<i64>>();

        let seeds_part2: Vec<_> = maps_and_seeds[0]
            .split(": ")
            .nth(1).unwrap()
            .split(" ")
            .map(|seed| {
                seed.parse::<i64>().unwrap()
            })
            .collect();
        let mut seeds_new: Vec<i64> = Default::default();
        for i in 0..seeds_part2.iter().count() {
            let seed = seeds_part2[i];
            if i % 2 == 0 {
                for n in seed+1..seed + seeds_part2[i + 1] {
                    seeds_new.push(n);
                }
            }
        }
        let mut real_seeds_part2: Vec<_> = seeds_part2.iter().enumerate().take_while(|(i, _)| i % 2 == 0).map(|(_, x)| *x).collect();
        real_seeds_part2.extend(seeds_new);

        let mut maps: Maps = Default::default();

        // WARNING: many maps
        let mut map_kind: MapKind = Default::default();
        maps_and_seeds.iter().skip(1).collect::<Vec<&&str>>().iter().for_each(|mappings| {
            mappings
                .split("map:\n")
                .nth(1)
                .unwrap()
                .trim()
                .split("\n")
                .collect::<Vec<&str>>()
                .iter()
                // WARNING: many ranges of one map
                .for_each(|mapping| {
                    let d_s_l: Vec<i64> = mapping
                        .split(" ")
                        .collect::<Vec<&str>>()
                        .iter()
                        .map(|str| str.parse::<i64>().unwrap())
                        .collect();

                    let dest = d_s_l.iter().nth(0).unwrap();
                    let source = d_s_l.iter().nth(1).unwrap();
                    let length = d_s_l.iter().nth(2).unwrap();

                    let map = Map { dest: *dest, source: *source, length: *length };

                    map_kind.kind.push(map);
                });
            maps.map_kinds.push(map_kind.clone());
            map_kind.kind.clear();
        });

        // dbg!(&seeds);

        // for f in 0..maps.map_kinds.iter().count() {
        //     // println!("Mapping {}", f + 1);
        //     let map_kind = &maps.map_kinds[f];
        //     for i in 0..seeds.iter().count() {
        //         let seed = seeds[i];
        //         for map in &map_kind.kind {
        //             if (map.source..map.source + map.length).contains(&seed) {
        //                 let mapped = map.dest + seed - map.source;
        //                 // dbg!(mapped);
        //                 seeds[i] = mapped;
        //             }
        //         }
        //     }
        // }

        // let lowest_part1 = seeds.iter().min().unwrap();
        // dbg!(lowest_part1);

        for f in 0..maps.map_kinds.iter().count() {
            // println!("Mapping {}", f + 1);
            let map_kind = &maps.map_kinds[f];
            for i in 0..real_seeds_part2.iter().count() {
                let seed = real_seeds_part2[i];
                for map in &map_kind.kind {
                    if (map.source..map.source + map.length).contains(&seed) {
                        let mapped = map.dest + seed - map.source;
                        // dbg!(mapped);
                        real_seeds_part2[i] = mapped;
                    }
                }
            }
        }

        let lowest_part2 = real_seeds_part2.iter().min().unwrap();
        dbg!(lowest_part2);
    }
}


fn main() {
    println!("Hello, world!");
    let day_5 = DAY5::construct();
    day_5.map_to_location();
}
