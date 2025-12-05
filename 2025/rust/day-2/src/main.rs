use std::path::PathBuf;

mod part_1 {
    use super::*;

    pub(crate) fn solve(ranges: Vec<Range>) -> usize {
        let mut sum = 0;

        for range in ranges {
            for id in range.from..=range.to {
                if !is_valid_id(id) {
                    sum += id;
                }
            }
        }

        sum
    }

    fn is_valid_id(id: usize) -> bool {
        let id = id.to_string();

        if id.len() % 2 == 1 {
            return true
        }

        let (first, second) = id.split_at(id.len() / 2);

        first != second
    }
}

mod part_2 {
    use super::*;

    pub(crate) fn solve(ranges: Vec<Range>) -> usize {
        let mut sum = 0;


        for range in ranges {
            let mut range_max = 0;
            let mut range_checks = 0;

            for id in range.from..=range.to {
                let (valid, max, checks) = is_valid_id(id);
                range_max += max;
                range_checks += checks;
                if !valid {
                    sum += id;
                }
            }
            println!("{range_checks}/{range_max} ({})", ((range_checks as f64 / range_max as f64) * 100.0) as usize);
        }

        sum
    }

    fn is_valid_id(id: usize) -> (bool, usize, usize) {
        let id = id.to_string();

        let mut max = 0;
        let mut checks = 0;

        'length: for part_length in (1..=id.len()/2).rev() {
            if id.len() % part_length != 0 {
                continue;
            }

            let part_count = id.len()/ part_length;
            let mut previous_part = None;

            max += part_count;

            for part_idx in 0..part_count {
                checks += 1;
                let start_idx = part_idx * part_length;
                let end_idx = part_idx * part_length + part_length;

                let current_part = &id[start_idx..end_idx];

                match previous_part {
                    None => previous_part = Some(current_part),
                    Some(previous) => {
                        if previous != current_part {
                            continue 'length;
                        }
                    }
                }
            }

            return (false, max, checks);
        }

        (true, max, checks)
    }
}

struct Range {
    from: usize,
    to: usize,
}

fn read_input() -> Vec<Range> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("input.txt");
    let input = std::fs::read_to_string(path).expect("Unable to read file");

    let zero = '0';
    let parse = |str: &str| str.chars().skip_while(|c| c == &zero).collect::<String>().parse::<usize>().expect("Unable to parse number");

    input.split(",").map(|range| {
        let (from, to) = range.split_once("-").expect("unable to parse range");

        Range {
            from: parse(from),
            to: parse(to),
        }
    }).collect()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        Some("part-1") => {
            let input = read_input();
            let password = part_1::solve(input);
            println!("answer is: {password}");
        },
        Some("part-2") => {
            let input = read_input();
            let password = part_2::solve(input);
            println!("answer is: {password}");
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
        let ranges = vec![
            Range { from: 11, to: 22 },
            Range { from: 95, to: 115 },
            Range { from: 998, to: 1012 },
            Range { from: 1188511880, to: 1188511890 },
            Range { from: 222220, to: 222224 },
            Range { from: 1698522, to: 1698528 },
            Range { from: 446443, to: 446449 },
            Range { from: 38593856, to: 38593862 },
        ];

        assert_eq!(part_1::solve(ranges), 1227775554);
    }

    #[test]
    fn part_2() {
        let ranges = vec![
            Range { from: 11, to: 22 },
            Range { from: 95, to: 115 },
            Range { from: 998, to: 1012 },
            Range { from: 1188511880, to: 1188511890 },
            Range { from: 222220, to: 222224 },
            Range { from: 1698522, to: 1698528 },
            Range { from: 446443, to: 446449 },
            Range { from: 38593856, to: 38593862 },
            Range { from: 565653, to: 565659 },
            Range { from: 824824821, to: 824824827 },
            Range { from: 2121212118, to: 2121212124 },
        ];

        assert_eq!(part_2::solve(ranges), 4174379265);
    }
}