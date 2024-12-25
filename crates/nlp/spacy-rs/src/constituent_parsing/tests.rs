use super::*;
use crate::constituent_parsing::parser::SpacyConstituentParser;
use alien_seed::{attach::with_seed, AlienSeed};
use expect_test::expect_file;
use husky_path_utils::HuskyLangDevPaths;
use std::{path::PathBuf, sync::Arc};
#[test]
fn constituent_parsing_works() {
    use eterned::db::EternerDb;

    let seed = AlienSeed::new(0);
    let db = &EternerDb::default();
    let tokio_runtime = Arc::new(tokio::runtime::Runtime::new().unwrap());
    let parser = SpacyConstituentParser::new(
        db,
        tokio_runtime,
        PathBuf::from("tests-data/constituent_parsing/cache.json"),
    )
    .unwrap();
    with_seed(seed, || {
        let output = parser
            .parse("Let FORMULA1 be a topological space.".to_string())
            .unwrap();
        let json_output = serde_json::to_string_pretty(&output).unwrap();
        assert!(Path::new("tests-data/constituent_parsing").exists());
        assert!(Path::new("tests-data/constituent_parsing").is_dir());
        expect_file!("../../tests-data/constituent_parsing/output.json").assert_eq(&json_output);
    });
}
