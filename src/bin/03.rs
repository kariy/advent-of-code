fn get_item_priority(item: &char) -> u32 {
    let ascii = *item as u32;
    if ascii >= 65 && ascii < 91 {
        ascii - 38
    } else {
        ascii - 96
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut max = 0;

    for line in input.lines() {
        let mid = line.len() / 2;

        for item in line[..mid].chars() {
            if line[mid..].find(item).is_some() {
                max += get_item_priority(&item);
                break;
            }
        }
    }

    Some(max)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut max = 0;
    let mut lines = input.lines();

    while let (Some(line1), Some(line2), Some(line3)) = (lines.next(), lines.next(), lines.next()) {
        for item in line1.chars() {
            if let (Some(_), Some(_)) = (line2.find(item), line3.find(item)) {
                max += get_item_priority(&item);
                break;
            }
        }
    }

    Some(max)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
