use std::path::PathBuf;

fn main() {
    let dir = PathBuf::from("data/mini-husky/basic");
    assert!(dir.exists())
}
