pub fn part_one(input: &str) -> Option<u32> {
    let mut points = 0;
    for line in input.lines() {
        let play: Vec<&str> = line.split_whitespace().collect();

        points += match play[1] {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => unreachable!(),
        };

        points += match (play[0], play[1]) {
            ("A", "X") => 3,
            ("A", "Y") => 6,
            ("A", "Z") => 0,
            ("B", "X") => 0,
            ("B", "Y") => 3,
            ("B", "Z") => 6,
            ("C", "X") => 6,
            ("C", "Y") => 0,
            ("C", "Z") => 3,
            _ => unreachable!(),
        };
    }
    Some(points)
}

fn outcome_points(outcome: &str) -> u32 {
    match outcome {
        "X" => 0,
        "Y" => 3,
        "Z" => 6,
        _ => panic!(),
    }
}

fn play_points(play: &str) -> u32 {
    match play {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        _ => panic!(),
    }
}

fn to_play_if(opponent: &str, outcome: &str) -> &'static str {
    match (opponent, outcome) {
        ("A", "X") => "C",
        ("A", "Y") => "A",
        ("A", "Z") => "B",
        ("B", "X") => "A",
        ("B", "Y") => "B",
        ("B", "Z") => "C",
        ("C", "X") => "B",
        ("C", "Y") => "C",
        ("C", "Z") => "A",
        _ => panic!(),
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut points = 0;
    for line in input.lines() {
        let strat = line.split_whitespace().collect::<Vec<&str>>();
        let play = to_play_if(strat[0], strat[1]);
        points += play_points(play) + outcome_points(strat[1]);
    }
    Some(points)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(26));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(27));
    }
}
