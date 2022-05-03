use compile_time_db::{utils::test_all_modules, *};
use print_utils::p;
use std::path::Path;
use test_utils::TestResult;
use token::AbsSemanticToken;

pub(super) fn test_semantic_tokens(package_dir: &Path) -> TestResult {
    test_all_modules(
        package_dir,
        "semantic_tokens.txt",
        |compile_time, module| {
            let file = compile_time.module_file(module).unwrap();
            let ast_text = compile_time.ast_text(file).unwrap();
            AbsSemanticToken::to_semantic_tokens(&ast_text.semantic_tokens)
        },
    )
}
