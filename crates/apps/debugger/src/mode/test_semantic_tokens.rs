use super::*;
use compile_time_db::*;
use token::AbsSemanticToken;

pub(super) async fn test_semantic_tokens(pack_path: &Path, compile_time: &HuskyLangCompileTime) {
    type SemanticTokensTable = HashMap<String, Vec<AbsSemanticToken>>;

    let modules = compile_time.all_modules();
    let mut highlights_table = HashMap::<String, Vec<AbsSemanticToken>>::new();
    for module in modules {
        let file = compile_time.module_file(module).unwrap();
        let ast_text = compile_time.ast_text(file).unwrap();
        let semantic_tokens = ast_text.semantic_tokens.clone();
        assert!(highlights_table
            .insert(module.to_str(), semantic_tokens)
            .is_none());
    }
    compare_semantic_tokens_tables(highlights_table, pack_path);

    fn compare_semantic_tokens_tables(semantic_tokens_table: SemanticTokensTable, path: &Path) {
        let semantic_tokens_table_path = path.join("semantic_tokens_table.json");
        let semantic_tokens_table_on_disk: SemanticTokensTable =
            if !semantic_tokens_table_path.exists() {
                Default::default()
            } else {
                let text = fs::read_to_string(&semantic_tokens_table_path).unwrap();
                let v: serde_json::Value = serde_json::from_str(&text).unwrap();
                match serde_json::from_value(v) {
                    Ok(v) => v,
                    Err(e) => {
                        notify_deserialize_error(
                            semantic_tokens_table,
                            &text,
                            &e,
                            &semantic_tokens_table_path,
                        );
                        return;
                    }
                }
            };
        if semantic_tokens_table_on_disk != semantic_tokens_table {
            notify_change(
                semantic_tokens_table,
                semantic_tokens_table_on_disk,
                &semantic_tokens_table_path,
            )
        }
    }
}
