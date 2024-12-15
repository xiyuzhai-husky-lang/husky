use super::*;
use crate::constituent_parsing::parser::SpacyConstituentParser;
use expect_test::expect_file;
use husky_path_utils::HuskyLangDevPaths;

#[test]
fn constituent_parsing_works() {
    use eterned::db::EternerDb;

    let db = &EternerDb::default();
    let dev_paths = HuskyLangDevPaths::new();
    let python_lib_dir = python_src_dir(dev_paths.python_lib_dir()).join("constituent_parsing.py");
    let parser = SpacyConstituentParser::new(
        db,
        python_lib_dir,
        PathBuf::from("tests-data/constituent_parsing/cache.json"),
    )
    .unwrap();
    let output = parser
        .parse("Let FORMULA1 be a topological space.".to_string())
        .unwrap();
    let json_output = serde_json::to_string_pretty(&output).unwrap();
    assert!(Path::new("tests-data/constituent_parsing").exists());
    assert!(Path::new("tests-data/constituent_parsing").is_dir());
    expect_file!("../../tests-data/constituent_parsing/output.json").assert_eq(&json_output);
}
