use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let mut count = 0;

    for line in input.lines() {
        let reg = Regex::new(r"(\d*)-(\d*),(\d*)-(\d*)").unwrap();
        for cap in reg.captures_iter(line) {
            let p1 = (
                cap[1].parse::<u32>().unwrap(),
                cap[2].parse::<u32>().unwrap(),
            );
            let p2 = (
                cap[3].parse::<u32>().unwrap(),
                cap[4].parse::<u32>().unwrap(),
            );

            if p1.1 >= p2.1 && p1.0 <= p2.0 {
                count += 1;
                break;
            }

            if p2.1 >= p1.1 && p2.0 <= p1.0 {
                count += 1;
                break;
            }
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut count = 0;

    for line in input.lines() {
        let reg = Regex::new(r"(\d*)-(\d*),(\d*)-(\d*)").unwrap();
        for cap in reg.captures_iter(line) {
            let p1 = (
                cap[1].parse::<u32>().unwrap(),
                cap[2].parse::<u32>().unwrap(),
            );
            let p2 = (
                cap[3].parse::<u32>().unwrap(),
                cap[4].parse::<u32>().unwrap(),
            );

            if p1.1 >= p2.1 && p1.0 <= p2.1 {
                count += 1;
                break;
            }

            if p2.1 >= p1.1 && p2.0 <= p1.1 {
                count += 1;
                break;
            }
        }
    }

    Some(count)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
