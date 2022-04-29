use super::*;
use compile_time_db::*;
use lsp_types::SemanticToken;
use token::AbsSemanticToken;

pub(super) async fn test_semantic_tokens(pack_path: &Path, compile_time: &HuskyLangCompileTime) {
    let modules = compile_time.all_modules();
    let mut abs_semantic_tokens_table = HashMap::<String, Vec<AbsSemanticToken>>::new();
    let mut semantic_tokens_table = HashMap::<String, Vec<SemanticToken>>::new();
    for module in modules {
        let file = compile_time.module_file(module).unwrap();
        let ast_text = compile_time.ast_text(file).unwrap();
        let abs_semantic_tokens = ast_text.semantic_tokens.clone();
        assert!(semantic_tokens_table
            .insert(
                module.to_str(),
                AbsSemanticToken::to_semantic_tokens(&abs_semantic_tokens)
            )
            .is_none());
        assert!(abs_semantic_tokens_table
            .insert(module.to_str(), abs_semantic_tokens)
            .is_none());
    }
    compare_saved_data(
        &abs_semantic_tokens_table,
        &pack_path.join("abs_semantic_tokens_table.json"),
    );
    compare_saved_data(
        &semantic_tokens_table,
        &pack_path.join("semantic_tokens_table.json"),
    );
}
