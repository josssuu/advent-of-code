use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::str::FromStr;

mod part_1 {
    use super::*;

    pub(crate) fn solve(rotations: Vec<Rotation>) -> i32 {
        let mut password = 0;
        let max_clicks = 100;
        let mut current = 50i32;

        for rotation in rotations {
            match rotation.direction {
                Direction::LEFT => current -= rotation.clicks as i32,
                Direction::RIGHT => current += rotation.clicks as i32,
            }

            current = current % max_clicks;

            if current == 0 {
                password += 1;
            }
        }

        password
    }
}

mod part_2 {
    use super::*;

    pub(crate) fn solve(rotations: Vec<Rotation>) -> usize {
        let mut password = 0;
        let max_clicks = 100;
        let mut current = 50;

        for rotation in rotations {
            if rotation.clicks == 0 {
                continue;
            }

            let full_rotations = rotation.clicks / max_clicks;
            let remaining_clicks = rotation.clicks % max_clicks;

            let additional_rotations = if remaining_clicks > 0 {
                match rotation.direction {
                    Direction::LEFT => {
                        let rotations = if current == 0 || current > remaining_clicks { 0 } else { 1 };
                        current = (max_clicks + current - remaining_clicks) % max_clicks;
                        rotations
                    }
                    Direction::RIGHT => {
                        current += remaining_clicks;
                        let rotations = current / max_clicks;
                        current %= max_clicks;
                        rotations
                    }
                }
            } else { 0 };

            password += full_rotations + additional_rotations;
        }

        password
    }
}

enum Direction { LEFT, RIGHT }

struct Rotation {
    direction: Direction,
    clicks: usize,
}

fn read_input() -> Vec<Rotation> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("input.txt");
    let file = File::open(path).expect("Unable to open file");
    let reader = BufReader::new(file);

    reader.lines().map(|line| {
        let line = line.unwrap();
        let mut chars = line.chars();

        Rotation {
            direction: match chars.next().unwrap() {
                'L' => Direction::LEFT,
                'R' => Direction::RIGHT,
                _ => panic!("Unknown direction"),
            },
            clicks: usize::from_str(&chars.collect::<String>()).unwrap(),
        }
    }).collect()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        Some("part-1") => {
            let input = read_input();
            let password = part_1::solve(input);
            println!("password is: {password}");
        },
        Some("part-2") => {
            let input = read_input();
            let password = part_2::solve(input);
            println!("password is: {password}");
        }
        _ => {
            eprintln!("Usage: cargo run -- <part-1|part-2>");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let rotations = vec![
            Rotation {direction: Direction::LEFT, clicks: 68},
            Rotation {direction: Direction::LEFT, clicks: 30},
            Rotation {direction: Direction::RIGHT, clicks: 48},
            Rotation {direction: Direction::LEFT, clicks: 5},
            Rotation {direction: Direction::RIGHT, clicks: 60},
            Rotation {direction: Direction::LEFT, clicks: 55},
            Rotation {direction: Direction::LEFT, clicks: 1},
            Rotation {direction: Direction::LEFT, clicks: 99},
            Rotation {direction: Direction::RIGHT, clicks: 14},
            Rotation {direction: Direction::LEFT, clicks: 82},
        ];

        assert_eq!(part_1::solve(rotations), 3);
    }

    #[test]
    fn part_2() {
        let rotations = vec![
            Rotation {direction: Direction::LEFT, clicks: 68},
            Rotation {direction: Direction::LEFT, clicks: 30},
            Rotation {direction: Direction::RIGHT, clicks: 48},
            Rotation {direction: Direction::LEFT, clicks: 5},
            Rotation {direction: Direction::RIGHT, clicks: 60},
            Rotation {direction: Direction::LEFT, clicks: 55},
            Rotation {direction: Direction::LEFT, clicks: 1},
            Rotation {direction: Direction::LEFT, clicks: 99},
            Rotation {direction: Direction::RIGHT, clicks: 14},
            Rotation {direction: Direction::LEFT, clicks: 82},
        ];

        assert_eq!(part_2::solve(rotations), 6);
    }

    #[test]
    fn part_2_r1000() {
        let rotations = vec![
            Rotation {direction: Direction::RIGHT, clicks: 1000},
        ];

        assert_eq!(part_2::solve(rotations), 10);
    }

    #[test]
    fn part_2_3() {
        let rotations = vec![
            Rotation {direction: Direction::LEFT, clicks: 50},
        ];

        assert_eq!(part_2::solve(rotations), 1);
    }

    #[test]
    fn part_2_4() {
        let rotations = vec![
            Rotation {direction: Direction::LEFT, clicks: 50},
            Rotation {direction: Direction::RIGHT, clicks: 100},
        ];

        assert_eq!(part_2::solve(rotations), 2);
    }

    #[test]
    fn part_2_5() {
        let rotations = vec![
            Rotation {direction: Direction::LEFT, clicks: 50},
            Rotation {direction: Direction::LEFT, clicks: 100},
        ];

        assert_eq!(part_2::solve(rotations), 2);
    }

    #[test]
    fn part_2_6() {
        let rotations = vec![
            Rotation {direction: Direction::RIGHT, clicks: 50},
        ];

        assert_eq!(part_2::solve(rotations), 1);
    }

    #[test]
    fn part_2_7() {
        let rotations = vec![
            Rotation {direction: Direction::RIGHT, clicks: 51},
        ];

        assert_eq!(part_2::solve(rotations), 1);
    }

    #[test]
    fn part_2_8() {
        let rotations = vec![
            Rotation {direction: Direction::RIGHT, clicks: 51},
            Rotation {direction: Direction::LEFT, clicks: 1},
        ];

        assert_eq!(part_2::solve(rotations), 2);
    }

    #[test]
    fn part_2_9() {
        let rotations = vec![
            Rotation {direction: Direction::RIGHT, clicks: 50},
            Rotation {direction: Direction::LEFT, clicks: 0},
        ];

        assert_eq!(part_2::solve(rotations), 1);
    }

    #[test]
    fn part_2_10() {
        let rotations = vec![
            Rotation {direction: Direction::RIGHT, clicks: 50},
            Rotation {direction: Direction::RIGHT, clicks: 0},
        ];

        assert_eq!(part_2::solve(rotations), 1);
    }
}