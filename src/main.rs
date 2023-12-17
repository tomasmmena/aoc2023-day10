use std::env;
use std::fs;
use std::io::{self, BufRead};

const MAX_STEPS: usize = 100_000;

enum Tile {
    NoPipe,
    VPipe,
    HPipe,
    LPipe,
    JPipe,
    SevenPipe,
    FPipe,
    StartingPosition
}

/// This implementation makes a couple of assumptions:
///  - There is no more than one starting point, if there were more than one and two where contiguous, this would result in a stack overflow.
///  - This will probably blow up due to an underflow if called on a tile on the left or top edges of the map that extends a pipe out of bounds.
fn get_connecting_tiles(x: usize, y: usize, pipes: &Vec<Vec<Tile>>) -> Vec<(usize, usize)> {
    match pipes[y][x] {
        Tile::FPipe => vec![(x + 1, y), (x, y + 1)],
        Tile::HPipe => vec![(x - 1, y), (x + 1, y)],
        Tile::JPipe => vec![(x - 1, y), (x, y - 1)],
        Tile::SevenPipe => vec![(x - 1, y), (x, y + 1)],
        Tile::LPipe => vec![(x, y - 1), (x + 1, y)],
        Tile::VPipe => vec![(x, y - 1), (x, y + 1)],
        Tile::StartingPosition => {
            vec![(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)]
                .into_iter()
                .filter(|(nx, ny)| get_connecting_tiles(*nx, *ny, pipes).contains(&(x, y)))
                .collect()
        },
        Tile::NoPipe => vec![]
    }
}


fn traverse(starting_point: (usize, usize), data: &Vec<Vec<Tile>>) -> usize {
    let mut current_position = starting_point.clone();
    let mut last_position = current_position.clone();
    let mut steps: usize = 0;
    while current_position == last_position || current_position != starting_point {
        let next_position = get_connecting_tiles(current_position.0, current_position.1, &data)
            .into_iter()
            .filter(|pos| *pos != last_position)
            .next()
            .unwrap();
        last_position = current_position;
        current_position = next_position;
        steps += 1;

        if steps > MAX_STEPS { panic!("Max steps reached!") }
    }
    steps
}


fn main() {
    let path = env::args().nth(1).expect("Missing required parameter path!");

    let mut starting_point: (usize, usize) = (0, 0);
    let data: Vec<Vec<Tile>> = io::BufReader::new(
        fs::File::open(path).expect("Could not open file!"))
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let text = line.expect("Could not read line!");
            text.chars().enumerate().map(|(j, c)| match c {
                'S' => {
                    starting_point = (j, i);
                    Tile::StartingPosition
                },
                '-' => Tile::HPipe,
                '|' => Tile::VPipe,
                'F' => Tile::FPipe,
                '7' => Tile::SevenPipe,
                'J' => Tile::JPipe,
                'L' => Tile::LPipe,
                _ => Tile::NoPipe,
            }).collect()
        })
        .collect();

    println!("Starting position is {}, {}", starting_point.0, starting_point.1);

    let steps = traverse(starting_point, &data);

    println!("Size of loop: {}", steps);
    println!("Max distance: {}", steps / 2);

}


#[cfg(test)]
mod tests {
    use crate::get_connecting_tiles;
    use crate::traverse;
    use crate::Tile;


    fn get_test_data() -> Vec<Vec<Tile>> {
        vec![
            vec![Tile::NoPipe, Tile::NoPipe, Tile::NoPipe, Tile::NoPipe, Tile::NoPipe],
            vec![Tile::NoPipe, Tile::FPipe, Tile::StartingPosition, Tile::SevenPipe, Tile::NoPipe],
            vec![Tile::NoPipe, Tile::VPipe, Tile::NoPipe, Tile::VPipe, Tile::NoPipe],
            vec![Tile::NoPipe, Tile::LPipe, Tile::HPipe, Tile::JPipe, Tile::NoPipe],
            vec![Tile::NoPipe, Tile::NoPipe, Tile::NoPipe, Tile::NoPipe, Tile::NoPipe],
        ]
    }

    #[test]
    fn test_get_connecting_tiles() {
        let test_map = get_test_data();

        // test F pipe
        assert!(get_connecting_tiles(1, 1, &test_map).contains(&(2, 1)));
        assert!(get_connecting_tiles(1, 1, &test_map).contains(&(1, 2)));
        assert!(get_connecting_tiles(1, 1, &test_map).len() == 2);

        // test V pipe
        assert!(get_connecting_tiles(2, 1, &test_map).contains(&(1, 1)));
        assert!(get_connecting_tiles(2, 1, &test_map).contains(&(3, 1)));
        assert!(get_connecting_tiles(2, 1, &test_map).len() == 2);

        // test L pipe
        assert!(get_connecting_tiles(3, 1, &test_map).contains(&(2, 1)));
        assert!(get_connecting_tiles(3, 1, &test_map).contains(&(3, 2)));
        assert!(get_connecting_tiles(3, 1, &test_map).len() == 2);

        // test H pipe
        assert!(get_connecting_tiles(3, 2, &test_map).contains(&(3, 1)));
        assert!(get_connecting_tiles(3, 2, &test_map).contains(&(3, 3)));
        assert!(get_connecting_tiles(3, 2, &test_map).len() == 2);

        // test J pipe
        assert!(get_connecting_tiles(3, 3, &test_map).contains(&(3, 2)));
        assert!(get_connecting_tiles(3, 3, &test_map).contains(&(2, 3)));
        assert!(get_connecting_tiles(3, 3, &test_map).len() == 2);

        // test 7 pipe
        assert!(get_connecting_tiles(1, 3, &test_map).contains(&(2, 3)));
        assert!(get_connecting_tiles(1, 3, &test_map).contains(&(1, 2)));
        assert!(get_connecting_tiles(1, 3, &test_map).len() == 2);

        // test S pipe
        assert!(get_connecting_tiles(1, 2, &test_map).contains(&(1, 1)));
        assert!(get_connecting_tiles(1, 2, &test_map).contains(&(1, 3)));
        assert!(get_connecting_tiles(1, 2, &test_map).len() == 2);
    }

    #[test]
    fn test_traverse() {
        let test_map = get_test_data();

        assert_eq!(traverse((1, 2), &test_map), 8);
    }

}
