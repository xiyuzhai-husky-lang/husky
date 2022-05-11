use compile_time_db::{utils::test_all_source_files, *};
use print_utils::p;
use std::path::Path;
use test_utils::TestResult;
use token::AbsSemanticToken;

pub(super) fn test_semantic_tokens(package_dir: &Path) -> TestResult {
    test_all_source_files(package_dir, "semantic_tokens.txt", |compile_time, file| {
        let ast_text = compile_time.ast_text(file).unwrap();
        AbsSemanticToken::to_semantic_tokens(&ast_text.semantic_tokens)
    })
}
