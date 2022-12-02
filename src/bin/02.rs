use advent_of_code::read_file_to_arr;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
enum Assignement {
    WIN,
    LOSE,
    DRAW,
}

impl Assignement {
    fn from_char(s: char) -> Assignement {
        match s {
            'Z' => Assignement::WIN,
            'X' => Assignement::LOSE,
            'Y' => Assignement::DRAW,
            _ => panic!("Invalid assignement"),
        }
    }

    fn score(&self) -> i32 {
        match self {
            Assignement::WIN => 6,
            Assignement::LOSE => 0,
            Assignement::DRAW => 3,
        }
    }

    fn move_to_play(&self, opp_move: &Moves) -> Moves {
        match self {
            Assignement::WIN => match opp_move {
                Moves::ROCK => Moves::PAPER,
                Moves::PAPER => Moves::SCISSORS,
                Moves::SCISSORS => Moves::ROCK,
            },
            Assignement::LOSE => match opp_move {
                Moves::ROCK => Moves::SCISSORS,
                Moves::PAPER => Moves::ROCK,
                Moves::SCISSORS => Moves::PAPER,
            },
            Assignement::DRAW => match opp_move {
                Moves::ROCK => Moves::ROCK,
                Moves::PAPER => Moves::PAPER,
                Moves::SCISSORS => Moves::SCISSORS,
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Moves {
    ROCK,
    PAPER,
    SCISSORS,
}

impl Moves {
    fn from_char(s: char) -> Moves {
        match s {
            'A' | 'X' => Moves::ROCK,
            'B' | 'Y' => Moves::PAPER,
            'C' | 'Z' => Moves::SCISSORS,
            _ => panic!("Invalid move"),
        }
    }

    fn score(&self) -> i32 {
        match self {
            Moves::ROCK => 1,
            Moves::PAPER => 2,
            Moves::SCISSORS => 3,
        }
    }

    fn score_for_move(&self, opp_move: &Moves) -> i32 {
        match self {
            Moves::ROCK => match opp_move {
                Moves::ROCK => 3,
                Moves::PAPER => 0,
                Moves::SCISSORS => 6,
            },
            Moves::PAPER => match opp_move {
                Moves::ROCK => 6,
                Moves::PAPER => 3,
                Moves::SCISSORS => 0,
            },
            Moves::SCISSORS => match opp_move {
                Moves::ROCK => 0,
                Moves::PAPER => 6,
                Moves::SCISSORS => 3,
            },
        }
    }
}

fn main() {
    let inputs = read_file_to_arr("inputs", 2);

    let mut scores = HashMap::new();
    scores.insert('W', 6);
    scores.insert('D', 3);
    scores.insert('L', 0);
    scores.insert('X', 1);
    scores.insert('Y', 2);
    scores.insert('Z', 3);
    scores.insert('A', 1);
    scores.insert('B', 2);
    scores.insert('C', 3);

    let instructions: Vec<(Moves, Moves)> = inputs
    .iter()
    .map(|line| {
        line.split_whitespace()
            .map(|s| s.chars().nth(0).unwrap())
            .filter(|c| !c.is_whitespace())
            .collect()
    })
    .map(|line: Vec<char>| (Moves::from_char(line[0]), Moves::from_char(line[1])))
    .collect();

    let mut total: i32 = instructions
        .iter()
        .map(|(m1, m2)| Moves::score_for_move(m2, m1) + m2.score())
        .sum();

    println!("{}", total);

    let advanced_instructions: Vec<(Moves, Assignement)> = inputs
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.chars().nth(0).unwrap())
                .filter(|c| !c.is_whitespace())
                .collect()
        })
        .map(|line: Vec<char>| (Moves::from_char(line[0]), Assignement::from_char(line[1])))
        .collect();


    total = advanced_instructions
        .iter()
        .map(|(opp_move, assignement)| {
            Assignement::score(assignement)
                + Moves::score(&Assignement::move_to_play(assignement, opp_move))
        })
        .sum();

    println!("{}", total);
}
