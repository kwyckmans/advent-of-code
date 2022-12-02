use advent_of_code::read_file_to_arr;
use std::collections::HashMap;

fn main() {
    let inputs = read_file_to_arr("inputs", 2);

    let mut scores = HashMap::new();
    scores.insert('W', 6);
    scores.insert('D', 3);
    scores.insert('L', 0);
    scores.insert('X', 1);
    scores.insert('Y', 2);
    scores.insert('Z', 3);


    let res: Vec<Vec<char>> = inputs.iter().map(|line| line.split_whitespace().map(|s| s.chars().nth(0).unwrap()).filter(|c| !c.is_whitespace() ).collect()).collect();

    let total = res.iter()
        .map(|moves| calculate_score(moves[0], moves[1], &scores))
        .sum::<i32>();

    println!("{}",total);
    // res.into_iter().for_each(|mut line| {
    //     println!("{:?}", line);
    // });

    // println!("split input: {:?}", res);
}

fn calculate_score(opp_move: char, player_move: char, scores: &HashMap<char, i32>) -> i32 {
    println!("Opponent move {}, player move {}", opp_move, player_move);
    match (opp_move, player_move) {
        ('A', 'X') => scores[&'D'] + scores[&player_move],
        ('A', 'Y') => scores[&'W'] + scores[&player_move],
        ('A', 'Z') => scores[&'L'] + scores[&player_move],
        ('B', 'X') => scores[&'L'] + scores[&player_move],
        ('B', 'Y') => scores[&'D'] + scores[&player_move],
        ('B', 'Z') => scores[&'W'] + scores[&player_move],
        ('C', 'X') => scores[&'W'] + scores[&player_move],
        ('C', 'Y') => scores[&'L'] + scores[&player_move],
        ('C', 'Z') => scores[&'D'] + scores[&player_move],
        _ => 0,
    }
}
