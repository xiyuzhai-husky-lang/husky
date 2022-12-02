#[salsa::db(WordJar)]
#[derive(Default)]
struct MimicDB {
    storage: Storage<Self>,
}

impl Database for MimicDB {}

use crate::*;
use husky_expect_test_utils::*;
use husky_word::WordJar;
use salsa::{Database, Storage};

#[test]
fn tokenize_works() {
    expect_test_husky_to_rust("", &tokenize_debug);

    fn tokenize_debug(text: &str) -> String {
        todo!()
        // format!("{:#?}", MimicDB::default().tokenize_line(text))
    }
}
