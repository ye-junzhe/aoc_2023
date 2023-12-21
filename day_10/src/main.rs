use std::{fs, ops::Add, usize, u64};

#[derive(Default, Debug)]
struct DAY10 {
    data: String,
}

#[derive(Default, Debug, Clone, Copy)]
enum TileType {
    #[default] Ground,
    StartingPosition,
    VerticalPipe,
    HorizontalPipe,
    NorthEastBend,
    NorthWestBend,
    SouthEastBend,
    SouthWestBend,
}

impl PartialEq for TileType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TileType::Ground, TileType::Ground) => true,
            (TileType::StartingPosition, TileType::StartingPosition) => true,
            (TileType::VerticalPipe, TileType::VerticalPipe) => true,
            (TileType::HorizontalPipe, TileType::HorizontalPipe) => true,
            (TileType::NorthEastBend, TileType::NorthEastBend) => true,
            (TileType::NorthWestBend, TileType::NorthWestBend) => true,
            (TileType::SouthEastBend, TileType::SouthEastBend) => true,
            (TileType::SouthWestBend, TileType::SouthWestBend) => true,
            _ => false,
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
struct Tile {
    coord: (u64, u64),
    tile_type: TileType,
}

#[derive(Default, Debug)]
struct Maze {
    start: Tile,
    grids: Vec<Tile>,
}

const FILE_PATH: &str = "input.txt";

fn get_maze_wall(
    start_coords: Vec<(u64, u64)>,
    row: u64,
    column: u64,
    map: &Vec<Tile>,
    tile_tree: &mut Vec<Vec<Tile>>,
    maze_wall: &mut Vec<(u64, u64)>)
-> Vec<(u64, u64)> {

    let mut new_starting_tiles: Vec<Tile> = Default::default();

    for start_coord in start_coords {
        let up = (start_coord.0.checked_sub(1), start_coord.1);
        if up.0.is_some() && !maze_wall.iter().find(|(y, x)| *y == up.0.unwrap() && *x == up.1).is_some() {
            // dbg!(&tiles[(up.0.unwrap() * elements_per_line + up.1) as usize].tile_type);
            match map[(up.0.unwrap() * column + up.1) as usize].tile_type {
                TileType::SouthWestBend => {
                    let up = (up.0.unwrap(), up.1);
                    new_starting_tiles.push(Tile { coord: up, tile_type: TileType::SouthWestBend });
                    maze_wall.push((up.0, up.1));
                }
                TileType::SouthEastBend => {
                    let up = (up.0.unwrap(), up.1);
                    new_starting_tiles.push(Tile { coord: up, tile_type: TileType::SouthEastBend });
                    maze_wall.push((up.0, up.1));
                }
                TileType::VerticalPipe => {
                    let up = (up.0.unwrap(), up.1);
                    new_starting_tiles.push(Tile { coord: up, tile_type: TileType::VerticalPipe });
                    maze_wall.push((up.0, up.1));
                }
                _ => (),
            }
        }

        let down = (start_coord.0.add(1), start_coord.1);
        if (0..row).contains(&down.0) && !maze_wall.iter().find(|(y, x)| *y == down.0 && *x == down.1 ).is_some() {
            // dbg!(&tiles[(down.0 * elements_per_line + down.1) as usize].tile_type);
            match map[(down.0 * column + down.1) as usize].tile_type {
                TileType::NorthWestBend => {
                    let down = (down.0, down.1);
                    new_starting_tiles.push(Tile { coord: down, tile_type: TileType::NorthWestBend });
                    maze_wall.push((down.0, down.1));
                }
                TileType::NorthEastBend => {
                    let down = (down.0, down.1);
                    new_starting_tiles.push(Tile { coord: down, tile_type: TileType::NorthEastBend });
                    maze_wall.push((down.0, down.1));
                }
                TileType::VerticalPipe=> {
                    let down = (down.0, down.1);
                    new_starting_tiles.push(Tile { coord: down, tile_type: TileType::VerticalPipe });
                    maze_wall.push((down.0, down.1));
                }
                _ => (),
            }
        }

        let left = (start_coord.0, start_coord.1.checked_sub(1));
        if left.1.is_some() && !maze_wall.iter().find(|(y, x)| *y == left.0 && *x == left.1.unwrap() ).is_some() {
            // dbg!(&tiles[(left.0 * elements_per_line + left.1.unwrap()) as usize].tile_type);
            match map[(left.0 * column + left.1.unwrap()) as usize].tile_type {
                TileType::NorthEastBend => {
                    let left = (left.0, left.1.unwrap());
                    new_starting_tiles.push(Tile { coord: left, tile_type: TileType::NorthEastBend });
                    maze_wall.push((left.0, left.1));
                }
                TileType::SouthEastBend => {
                    let left = (left.0, left.1.unwrap());
                    new_starting_tiles.push(Tile { coord: left, tile_type: TileType::SouthEastBend });
                    maze_wall.push((left.0, left.1));
                }
                TileType::HorizontalPipe => {
                    let left = (left.0, left.1.unwrap());
                    new_starting_tiles.push(Tile { coord: left, tile_type: TileType::HorizontalPipe });
                    maze_wall.push((left.0, left.1));
                }
                _ => (),
            }
        }

        let right = (start_coord.0, start_coord.1.add(1));
        if (0..column).contains(&right.1) && !maze_wall.iter().find(|(y, x)| *y == right.0 && *x == right.1 ).is_some() {
            // dbg!(&tiles[(right.0 * elements_per_line + right.1) as usize].tile_type);
            match map[(right.0 * column + right.1) as usize].tile_type {
                TileType::NorthWestBend => {
                    let right = (right.0, right.1);
                    new_starting_tiles.push(Tile { coord: right, tile_type: TileType::NorthWestBend});
                    maze_wall.push((right.0, right.1));
                }
                TileType::SouthWestBend=> {
                    let right = (right.0, right.1);
                    new_starting_tiles.push(Tile { coord: right, tile_type: TileType::SouthWestBend});
                    maze_wall.push((right.0, right.1));
                }
                TileType::HorizontalPipe => {
                    let right = (right.0, right.1);
                    new_starting_tiles.push(Tile { coord: right, tile_type: TileType::HorizontalPipe});
                    maze_wall.push((right.0, right.1));
                }
                _ => (),
            }
        }
    }

    let mut new_starting_positions: Vec<(u64, u64)> = Default::default();

    for tile in &new_starting_tiles {
        new_starting_positions.push(tile.coord);
    }

    tile_tree.push(new_starting_tiles.clone());

    new_starting_positions
}

impl DAY10 {
    fn construct() -> Self {
        DAY10 {
            data: fs::read_to_string(FILE_PATH).expect("Should have been able to read the file"),
        }
    }

    fn to_find(&self) {
        let mut maze: Maze = Default::default();
        let row: u64 = self.data.lines().count() as u64;
        let column: u64 = self.data.split("\n").nth(0).unwrap().chars().count() as u64;

        self.data.lines().enumerate().for_each(|(line_num, line)| {
            line.chars().enumerate().for_each(|(ind, c)| {
                let coord: (u64, u64) = (line_num as u64, ind as u64);
                let tile_type = match c {
                    '|' => TileType::VerticalPipe,
                    '-' => TileType::HorizontalPipe,
                    'L' => TileType::NorthEastBend,
                    'J' => TileType::NorthWestBend,
                    '7' => TileType::SouthWestBend,
                    'F' => TileType::SouthEastBend,
                    'S' => {
                        maze.start = Tile { coord, tile_type: TileType::Ground }; 
                        TileType::StartingPosition
                    },
                    _ => Default::default(),
                };
                maze.grids.push( Tile { coord, tile_type } );
            })
        });
        // dbg!(elements_per_line);
        let mut tile_tree: Vec<Vec<Tile>> = Default::default();
        let mut maze_wall: Vec<(u64, u64)> = Default::default();

        maze_wall.push(maze.start.coord);

        let mut start_coords: Vec<(u64, u64)> = Default::default();
        start_coords.push(maze.start.coord);

        let mut counter = 0;
        loop {
            let temp: Vec<Tile> = Default::default();
            if counter > 0 && tile_tree.iter().last().unwrap_or_else(|| &temp).iter().count() == 0 {
                tile_tree.remove(tile_tree.iter().count() - 1);
                break;
            }
            start_coords = get_maze_wall(start_coords, row, column, &maze.grids, &mut tile_tree, &mut maze_wall);

            counter += 1;
        }

        // NOTE: Point in polygon
        // https://en.wikipedia.org/wiki/Point_in_polygon
        maze.grids[(maze.start.coord.0 * column + maze.start.coord.1) as usize] = Tile { coord: maze.start.coord, tile_type: TileType::HorizontalPipe};

        dbg!(&maze_wall);

        let mut nest: Vec<(u64, u64)> = Default::default();

        for (y, line) in maze.grids.chunks(column as usize).into_iter().enumerate() {
            for (x, _tile) in line.iter().enumerate() {
                if maze_wall.contains(&(y as u64, x as u64)) {
                    continue;
                }
                let mut counter: u64 = 0;
                let mut wall_mode = false;
                let mut wall_count_left: u64 = 0;
                let mut turn_count: u64 = 0;

                for x_iter in 0..x as u64 {
                    if maze_wall.contains(&(y as u64, x_iter as u64)) {
                        wall_count_left += 1;
                    }
                }
                println!("=====================");
                println!("Y at {}", y);
                println!("X at {}", x);
                for x_iter in x as u64..column {
                    if maze_wall.contains(&(y as u64, x_iter as u64)) &&
                    ( maze.grids[y * column as usize + x_iter as usize].tile_type == TileType::SouthWestBend // 7
                    || maze.grids[y * column as usize + x_iter as usize].tile_type == TileType::SouthEastBend // F
                    || maze.grids[y * column as usize + x_iter as usize].tile_type == TileType::VerticalPipe ) // |
                    {
                        turn_count += 1;
                    }
                    // if !wall_mode {
                    //     if maze_wall.contains(&(y as u64, x_iter as u64)) {
                    //         wall_mode = true;
                    //         // counter += 1;
                    //         continue;
                    //     } else {
                    //         continue;
                    //     }
                    // } else {
                    //     if !maze_wall.contains(&(y as u64, x_iter as u64)) {
                    //         wall_mode = false;
                    //         counter += 1;
                    //         continue;
                    //     } else {
                    //         continue;
                    //     }
                    // }
                }
                dbg!(turn_count);

                if turn_count % 2 != 0 && wall_count_left > 0
                {
                    nest.push((y as u64, x as u64));
                    // println!("y: {}, x: {}", y, x);
                    dbg!(counter);
                }
            }
        }
        let tree_levels_part1_answer = tile_tree.iter().count();
        dbg!(tree_levels_part1_answer);

        dbg!(&nest);
        let nest_count_part2_answer = nest.iter().count();
        dbg!(nest_count_part2_answer);
    }

}

fn main() {
    let day_10 = DAY10::construct();
    day_10.to_find();
}
