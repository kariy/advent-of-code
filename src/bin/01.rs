pub fn part_one(input: &str) -> Option<u32> {
    let mut max = 0;
    let mut temp = 0;
    let mut lines = input.lines();

    loop {
        let line = lines.next();

        if line == Some("") || line.is_none() {
            if temp > max {
                max = temp;
            } else if line.is_none() {
                return Some(max);
            }

            temp = 0;
        } else {
            temp += line.unwrap().parse::<u32>().unwrap();
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let mut t: [u32; 3] = [0; 3];
    let mut temp = 0;

    loop {
        let line = lines.next();

        if line == Some("") || line.is_none() {
            let max = t[0] + t[1] + t[2];
            let a = temp + t[1] + t[2];
            let b = t[0] + temp + t[2];
            let c = t[0] + t[1] + temp;

            if a >= max && a >= b && a >= c {
                t[0] = temp;
            } else if b >= max && b >= a && b >= c {
                t[1] = temp;
            } else if c >= max && c >= a && c >= b {
                t[2] = temp;
            } else if line.is_none() {
                return Some(max);
            }
            temp = 0;
        } else {
            temp += line.unwrap().parse::<u32>().unwrap();
        }
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(56344));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(71445));
    }
}
