use std::collections::HashMap;

use regex::Regex;

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

fn parse_input(input: &str) -> Result<(Ship, Vec<MoveTx>)> {
    let mut ship = Ship::new();
    let mut tx_list: Vec<MoveTx> = Vec::new();
    let mut stacks: Vec<Vec<String>> = Vec::new();

    let stack_count_reg = Regex::new(
        r"\b(\d+)\s+\b(\d+)\s+\b(\d+)\s+\b(\d+)\s+\b(\d+)\s+\b(\d+)\s+\b(\d+)\s+\b(\d+)\s+\b(\d+)\s+",
    )?;
    let item_list_reg = Regex::new(
        r"(\[\w\]|\s{3})\s(\[\w\]|\s{3})\s(\[\w\]|\s{3})\s(\[\w\]|\s{3})\s(\[\w\]|\s{3})\s(\[\w\]|\s{3})\s(\[\w\]|\s{3})\s(\[\w\]|\s{3})\s(\[\w\]|\s{3})",
    )?;

    let mut is_down = false;
    let mut size_set = false;
    for line in input.lines() {
        // once it reaches the stacks label, it means it has reaches the end of the stack drawing
        if stack_count_reg.is_match(line) {
            is_down = true;
        }

        if !is_down {
            let items = item_list_reg.captures(line).unwrap();
            let mut items = items.iter();
            items.next();

            if !size_set {
                let total_size = items.clone().count();
                (0..total_size).for_each(|_| stacks.push(Vec::new()));
                size_set = true;
            }

            for (idx, item) in items.enumerate() {
                let i = item.unwrap().as_str();

                if i.eq("   ") {
                    continue;
                }

                stacks[idx].insert(
                    0,
                    { i.strip_prefix("[").unwrap().strip_suffix("]").unwrap() }.to_string(),
                );
            }
        } else {
            if MoveTx::regex().is_match(line) {
                tx_list.push(MoveTx::parse_tx_string(line)?);
            }
        }
    }

    for stack in stacks.iter() {
        ship.add_stack(stack);
    }

    Ok((ship, tx_list))
}

#[derive(Debug)]
struct MoveTx {
    from: usize,
    to: usize,
    total: u32,
}

impl MoveTx {
    pub(self) fn regex() -> regex::Regex {
        Regex::new(r"\bmove\s+(\d+)\s+from\s+(\d+)\s+to\s+(\d+)\b").unwrap()
    }

    fn parse_tx_string(tx: &str) -> Result<Self> {
        let reg = Self::regex();
        let caps = reg.captures(tx).expect("invalid tx format");

        Ok(Self {
            from: caps[2].parse::<usize>()? - 1,
            to: caps[3].parse::<usize>()? - 1,
            total: caps[1].parse::<u32>()?,
        })
    }
}

struct Ship(HashMap<usize, Vec<String>>);

#[allow(unused)]
impl Ship {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn move_cargo(&mut self, inst: &MoveTx) {
        for _ in 0..(inst.total as usize) {
            let item_to_move = self
                .0
                .get_mut(&inst.from)
                .expect("stack doesnt exists!")
                .pop()
                .expect("stack is empty!");

            self.0
                .get_mut(&inst.to)
                .expect("stack doesnt exists!")
                .push(item_to_move);
        }
    }

    pub fn add_stack(&mut self, stack: &Vec<String>) {
        let id = self.total();
        self.0.insert(id, stack.to_owned());
    }

    fn get(&self, index: usize) -> Option<&Vec<String>> {
        self.0.get(&index)
    }

    fn peek(&self, index: usize) -> Option<&String> {
        self.0.get(&index).unwrap().last()
    }

    fn total(&self) -> usize {
        self.0.len()
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let mut answer = String::new();
    let (mut ship, tx_list) = parse_input(input).unwrap();

    tx_list.iter().for_each(|tx| ship.move_cargo(tx));
    (0..ship.total()).for_each(|i| answer.push_str(ship.peek(i).unwrap()));

    Some(answer)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), None);
    }
}
