use advent_of_code::{ANSI_BOLD, ANSI_ITALIC, ANSI_RESET};
use std::process::Command;

fn main() {
    let total: f64 = (1..=1)
        .map(|day| execute_day(day))
        .sum();

    println!(
        "{}Total:{} {}{:.2}ms{}",
        ANSI_BOLD, ANSI_RESET, ANSI_ITALIC, total, ANSI_RESET
    );
}

fn execute_day(day: i32) -> f64{
    let day = format!("{:02}", day);

    let cmd = Command::new("cargo")
        .args(["run", "--release", "--bin", &day])
        .output()
        .unwrap();

    println!("----------");
    println!("{}| Day {} |{}", ANSI_BOLD, day, ANSI_RESET);
    println!("----------");

    let output = String::from_utf8(cmd.stdout).unwrap();
    let is_empty = output.is_empty();

    println!(
        "{}",
        if is_empty {
            "Not solved."
        } else {
            output.trim()
        }
    );

    if is_empty {
        0_f64
    } else {
        let exec_time = advent_of_code::parse_exec_time(&output);
        println!(
            "{}Execution time:{} {}{:.2}ms{}",
            ANSI_BOLD, ANSI_RESET, ANSI_ITALIC, exec_time, ANSI_RESET
        );
        exec_time
    }
}
