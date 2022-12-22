fn parse_input(input: &str) -> Vec<Vec<u32>> {
    let mut tree_matrix = Vec::new();
    for (idx, line) in input.lines().enumerate() {
        tree_matrix.push(Vec::new());
        for c in line.chars() {
            tree_matrix[idx].push(c.to_digit(10).unwrap());
        }
    }
    tree_matrix
}

fn check_tree_if_visible(matrix: &Vec<Vec<u32>>, row: usize, col: usize) -> bool {
    let mut top = true;
    let mut bot = true;
    let mut left = true;
    let mut right = true;

    let height = matrix[row][col];

    // let mut prev_in_row = 0;
    for r in 0..matrix.len() {
        // check in the same row
        if r == row {
            for c in 0..matrix[r].len() {
                let current = matrix[r][c];

                if c < col && current >= height {
                    left = false;
                } else if c > col && current >= height {
                    right = false;
                }
            }
        } else {
            let current = matrix[r][col];

            if r < row && current >= height {
                top = false;
            } else if r > row && current >= height {
                bot = false;
            }
        }
    }

    top || bot || left || right
}

pub fn part_one(input: &str) -> Option<u32> {
    let matrix = parse_input(input);
    // each row will have 2 trees visible on the outermost left and right edges
    let mut total_visible = (matrix.len() * 2) as u32 - 4;

    for (i, row) in matrix.iter().enumerate() {
        if i == 0 || i == matrix.len() - 1 {
            total_visible += row.len() as u32;
        } else {
            for j in 0..row.len() {
                if j == 0 || j == row.len() - 1 {
                    continue;
                }

                if check_tree_if_visible(&matrix, i, j) {
                    total_visible += 1;
                }
            }
        }
    }

    Some(total_visible)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), None);
    }
}
