use std::{
    collections::{HashMap, VecDeque},
};

pub fn part_one(input: &str) -> Option<String> {
    println!("{}", input);

    let (stacks_input, instructions) = input.split_once("\n\n").unwrap();

    let stack_indices = stacks_input.lines().last().unwrap();
    let mut stacks: HashMap<usize, VecDeque<char>> = HashMap::new();

    for stack in stacks_input.lines().rev() {
        for (i, c) in stack.chars().enumerate() {
            if c.is_alphabetic() {
                let stack_idx =
                    stack_indices.chars().nth(i).unwrap().to_digit(10).unwrap() as usize;
                stacks.entry(stack_idx).or_default().push_back(c);
            }
        }
    }

    for (i, stack) in &stacks {
        println!("{}: {:?}", i, stack);
    }

    let operations: Vec<Vec<usize>> = instructions
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .filter(|word| word.chars().all(|c| c.is_ascii_digit()))
                .map(|word| word.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    for op in operations {
        let amount = op[0];
        let from = op[1];
        let to = op[2];

        println!(
            "Moving {} crates from stack {} to stack {}",
            amount, from, to
        );

        for _ in 0..amount {
            let c = stacks.get_mut(&from).unwrap().pop_back().unwrap();
            stacks.get_mut(&to).unwrap().push_back(c);
        }
    }

    let mut solution = String::new();
    for i in 0..stacks.len() {
        solution.push(*stacks.get(&(i + 1)).unwrap().back().unwrap());
    }

    println!("{}", solution);
    Option::Some(solution)
}

pub fn part_two(input: &str) -> Option<String> {
    println!("{}", input);

    let (stacks_input, instructions) = input.split_once("\n\n").unwrap();

    let stack_indices = stacks_input.lines().last().unwrap();
    let mut stacks: HashMap<usize, VecDeque<char>> = HashMap::new();

    for stack in stacks_input.lines().rev() {
        for (i, c) in stack.chars().enumerate() {
            if c.is_alphabetic() {
                let stack_idx =
                    stack_indices.chars().nth(i).unwrap().to_digit(10).unwrap() as usize;
                stacks.entry(stack_idx).or_default().push_back(c);
            }
        }
    }

    for (i, stack) in &stacks {
        println!("{}: {:?}", i, stack);
    }

    let operations: Vec<Vec<usize>> = instructions
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .filter(|word| word.chars().all(|c| c.is_ascii_digit()))
                .map(|word| word.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    for op in operations {
        let amount = op[0];
        let from = op[1];
        let to = op[2];

        println!(
            "Moving {} crates from stack {} to stack {}",
            amount, from, to
        );
        
        let mut temp_stack = VecDeque::new();
        for _ in 0..amount {
            temp_stack.push_front(stacks.get_mut(&from).unwrap().pop_back().unwrap());
        }

        for c in temp_stack {
            stacks.get_mut(&to).unwrap().push_back(c);
        }
    }

    let mut solution = String::new();
    for i in 0..stacks.len() {
        solution.push(*stacks.get(&(i + 1)).unwrap().back().unwrap());
    }

    println!("{}", solution);
    Option::Some(solution)
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
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), None);
    }
}
