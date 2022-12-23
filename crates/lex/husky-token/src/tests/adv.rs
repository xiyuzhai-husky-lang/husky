use husky_adversarial_utils::rand_string;
use husky_print_utils::p;

use super::*;
use std::{panic::catch_unwind, path::PathBuf};

type Seed = u64;

macro_rules! check_env {
    () => {{
        match std::env::var("ADVERSARIAL") {
            Ok(n) => n.parse::<Seed>().unwrap(),
            Err(_) => return,
        }
    }};
}

fn catch_unwind_tokenize_snippet_debug(
    snippet: &str,
) -> Result<String, Box<dyn std::any::Any + Send + 'static>> {
    catch_unwind(|| tokenize_snippet_debug(snippet))
}

fn search_for_adversarials(name: &str, snippet_gen: impl Fn(Seed) -> String, n: Seed) {
    use indicatif::ProgressBar;

    let bar = ProgressBar::new(n);
    for i in 0..n {
        bar.inc(1);
        let snippet = snippet_gen(i);
        match catch_unwind_tokenize_snippet_debug(&snippet) {
            Ok(_) => (),
            Err(_) => {
                let cargo_manifest_dir: PathBuf =
                    std::env::var("CARGO_MANIFEST_DIR").unwrap().into();
                let path = cargo_manifest_dir.join(format!(
                    "tests/snippets/adversarials/{name}.test-inputs.json"
                ));
                p!(path);
                let mut snippets: Vec<String> =
                    match serde_json::from_str(&std::fs::read_to_string(&path).unwrap()) {
                        Ok(v) => v,
                        Err(_) => vec![],
                    };
                snippets.push(snippet);
                std::fs::write(path, serde_json::to_string_pretty(&snippets).unwrap());
                std::process::exit(1)
            }
        }
    }
    bar.finish();
}

fn escapes_gen(seed: Seed) -> String {
    format!("\"{}\"", rand_string(seed, 4, &["\\", "\"", "t", "a"]))
}

#[test]
fn tokenize_adversarial_works() {
    let n = check_env!();
    search_for_adversarials("escapes_gen", escapes_gen, n)
}
