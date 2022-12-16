fn foo(packets: &str, marker_size: usize) -> usize {
    let mut latch: usize = 0;
    for i in 0..(packets.len() - (marker_size - 1)) {
        if latch != 0 {
            break;
        }

        let chunk = &packets[i..(i + marker_size)];

        for j in 0..(marker_size - 1) {
            if chunk[(j + 1)..].contains(chunk.chars().nth(j).unwrap()) {
                break;
            }

            // if it manages to reach the last char meaning every char is distinct
            if j == marker_size - 2 {
                // the (i + marker_size)th character
                latch = i + marker_size;
            }
        }
    }

    latch
}

pub fn part_one(input: &str) -> Option<usize> {
    let latch = foo(input, 4);
    Some(latch)
}

pub fn part_two(input: &str) -> Option<usize> {
    let latch = foo(input, 14);
    Some(latch)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        let example1_input = input.lines().next().unwrap();
        assert_eq!(part_one(example1_input), Some(5));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        let mut input = input.lines();
        input.next();

        let example2_input = input.next().unwrap();
        assert_eq!(part_two(example2_input), Some(19));
    }
}
