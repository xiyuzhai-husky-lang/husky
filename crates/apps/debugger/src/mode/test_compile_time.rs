use super::*;
use compile_time_db::*;
use lsp_types::{SemanticToken, SemanticTokens};

pub(super) async fn test_compile_time(dir: PathBuf) {
    let pack_paths = collect_pack_dirs(dir);
    println!(
        "\n{}Running{} {} tests on compile time:",
        print_utils::CYAN,
        print_utils::RESET,
        pack_paths.len()
    );

    for pack_path in pack_paths {
        let mut compile_time = HuskyLangCompileTime::default();
        init_compile_time_from_dir(&mut compile_time, pack_path.to_path_buf());
        println!(
            "\n{}test{} {}",
            print_utils::CYAN,
            print_utils::RESET,
            pack_path.as_os_str().to_str().unwrap(),
        );
        test_semantic_tokens(&pack_path, &compile_time).await;
        test_diagnostics(&pack_path, &compile_time).await
    }
}

async fn test_semantic_tokens(pack_path: &Path, compile_time: &HuskyLangCompileTime) {
    type SemanticTokensTable = HashMap<String, SemanticTokens>;

    let modules = compile_time.all_modules();
    let mut highlights_table = HashMap::<String, SemanticTokens>::new();
    for module in modules {
        let file = compile_time.module_file(module).unwrap();
        let ast_text = compile_time.ast_text(file).unwrap();
        let semantic_tokens = ast_text.semantic_tokens.clone();
        assert!(highlights_table
            .insert(module.to_str(), semantic_tokens)
            .is_none());
    }
    compare_semantic_tokens_tables(highlights_table, pack_path);

    fn compare_semantic_tokens_tables(diagnostics_table: SemanticTokensTable, path: &Path) {
        let diagnostics_table_path = path.join("diagnostics_table.json");
        let diagnostics_table_on_disk: SemanticTokensTable = if !diagnostics_table_path.exists() {
            Default::default()
        } else {
            let text = fs::read_to_string(diagnostics_table_path).unwrap();
            let v: serde_json::Value = serde_json::from_str(&text).unwrap();
            serde_json::from_value(v).unwrap()
        };
        if diagnostics_table_on_disk != diagnostics_table {
            p!(diagnostics_table);
            p!(diagnostics_table_on_disk);
            todo!()
        } else {
            println!(
                "    {}result{}: {}ok{}",
                print_utils::CYAN,
                print_utils::RESET,
                print_utils::GREEN,
                print_utils::RESET,
            )
        }
    }
}

async fn test_diagnostics(pack_path: &Path, compile_time: &HuskyLangCompileTime) {
    type DiagnosticsTable = HashMap<String, Vec<Diagnostic>>;

    let modules = compile_time.all_modules();
    let mut diagnostics_table = HashMap::<String, Vec<Diagnostic>>::new();
    for module in modules {
        compile_time
            .diagnostic_reserve(module)
            .release(|diagnostics| {
                if diagnostics.len() > 0 {
                    assert!(diagnostics_table
                        .insert(module.to_str(), diagnostics.clone())
                        .is_none());
                }
            });
    }
    compare_diagnostics_tables(diagnostics_table, pack_path);

    fn compare_diagnostics_tables(diagnostics_table: DiagnosticsTable, path: &Path) {
        let diagnostics_table_path = path.join("diagnostics_table.json");
        let diagnostics_table_on_disk: DiagnosticsTable = if !diagnostics_table_path.exists() {
            Default::default()
        } else {
            let text = fs::read_to_string(diagnostics_table_path).unwrap();
            let v: serde_json::Value = serde_json::from_str(&text).unwrap();
            serde_json::from_value(v).unwrap()
        };
        if diagnostics_table_on_disk != diagnostics_table {
            p!(diagnostics_table);
            p!(diagnostics_table_on_disk);
            todo!()
        } else {
            println!(
                "    {}result{}: {}ok{}",
                print_utils::CYAN,
                print_utils::RESET,
                print_utils::GREEN,
                print_utils::RESET,
            )
        }
    }
}
