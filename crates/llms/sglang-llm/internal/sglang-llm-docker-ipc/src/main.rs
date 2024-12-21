use sglang_llm_ffi::chat::generate_text_batch;
use sglang_llm_prelude::greeting::IPC_GREETING;
use std::io::{self, BufRead};

fn main() {
    println!("{}", IPC_GREETING);

    for_each_stdin_line(|line| {
        let prompts: Vec<String> = serde_json::from_str(line).unwrap();
        eprintln!("prompts = {:#?}", prompts);
        let answers = generate_text_batch(prompts);
        eprintln!("answers = {:#?}", answers);
        todo!()
        // let numbers: Vec<i32> = line
        //     .split_whitespace()
        //     .filter_map(|s| s.parse().ok())
        //     .collect();

        // if numbers.len() >= 2 {
        //     let sum: i32 = numbers.iter().sum();
        //     println!("{}", sum);
        // }
    });
}

fn for_each_stdin_line<F, R>(mut f: F)
where
    F: FnMut(&str) -> R,
{
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut input = String::new();

    while reader.read_line(&mut input).unwrap() > 0 {
        let result = f(input.trim());
        input.clear();
    }
}
