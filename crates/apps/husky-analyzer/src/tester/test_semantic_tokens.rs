use husky_display_utils::HuskyDisplay;
use husky_test_utils::TestResult;

use lsp_types::SemanticToken;
use std::path::Path;

pub(super) fn test_semantic_tokens(_package_dir: &Path) -> TestResult {
    todo!()
    //     test_all_source_files(
    //         package_dir,
    //         "semantic_tokens.txt",
    //         |comptime, file| match comptime.ast_text(file) {
    //             Ok(ast_text) => AbsSemanticToken::to_semantic_tokens(&ast_text.semantic_tokens)
    //                 .into_iter()
    //                 .map(|st| SemanticTokenWrapper(st))
    //                 .collect(),
    //             Err(_) => Vec::new(),
    //         },
    //     )
}

#[derive(Debug, PartialEq, Eq)]
pub struct SemanticTokenWrapper(SemanticToken);

impl HuskyDisplay for SemanticTokenWrapper {
    fn write_inherent(
        &self,
        _config: husky_display_utils::HuskyDisplayConfig,
        result: &mut String,
    ) {
        use std::fmt::Write;
        write!(result, "{:?}", self.0).unwrap();
    }
}
