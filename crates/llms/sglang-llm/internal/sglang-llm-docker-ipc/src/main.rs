use sglang_llm_prelude::greeting::IPC_GREETING;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let mut input = String::new();

    println!("{}", IPC_GREETING);

    while reader.read_line(&mut input).unwrap() > 0 {
        let numbers: Vec<i32> = input
            .trim()
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        if numbers.len() >= 2 {
            let sum: i32 = numbers.iter().sum();
            println!("{}", sum);
        }

        input.clear();
    }
}
