pub fn repeat_line(input: &str, idx: usize) -> String {
    let mut s = String::new();
    for (i, line) in input.lines().enumerate() {
        s += line;
        if i < input.lines().count() - 1 {
            s += "\n";
        }

        if i == idx {
            s += line;
            if i < input.lines().count() - 1 {
                s += "\n";
            }
        }
    }
    return s;
}

#[test]
fn test() {
    let new_data = repeat_line(
        r#"a+b=c;
                x+y=z;
                b+c=w;
             "#,
        2,
    );
    assert_eq!(
        new_data,
        r#"a+b=c;
                x+y=z;
                b+c=w;
                b+c=w;
             "#
    )
}

use rand::{thread_rng, Rng};

// chatgpt only get half of it correct, has to use random_string, sad
pub fn rand_string(seed: u64, max_length: usize, pieces: &[&str]) -> String {
    let mut rng = thread_rng();
    // Generate a random length between 1 and max_length (inclusive)
    let length = rng.gen_range(1..(max_length + 1));
    generate(seed, length, pieces)
}

pub fn generate(seed: u64, length: usize, pieces: &[&str]) -> String {
    fastrand::seed(seed);
    if pieces.is_empty() {
        panic!("Provided charset is empty! It should contain at least one character");
    }

    let mut result = String::with_capacity(length);

    unsafe {
        for _ in 0..length {
            result += pieces[fastrand::usize(0..pieces.len())];
        }
    }

    result
}
