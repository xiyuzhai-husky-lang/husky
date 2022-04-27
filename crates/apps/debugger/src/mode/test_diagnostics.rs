use super::*;
use compile_time_db::*;

pub(super) async fn test_diagnostics(
    pack_path: &Path,
    compile_time: &HuskyLangCompileTime,
) -> bool {
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
    compare_diagnostics_tables(&diagnostics_table, pack_path);
    return diagnostics_table.len() > 0;

    fn compare_diagnostics_tables(diagnostics_table: &DiagnosticsTable, path: &Path) {
        let diagnostics_table_path = path.join("diagnostics_table.json");
        let diagnostics_table_on_disk: DiagnosticsTable = if !diagnostics_table_path.exists() {
            Default::default()
        } else {
            let text = fs::read_to_string(&diagnostics_table_path).unwrap();
            let v: serde_json::Value = serde_json::from_str(&text).unwrap();
            match serde_json::from_value(v) {
                Ok(v) => v,
                Err(e) => {
                    notify_deserialize_error(diagnostics_table, &text, &e, &diagnostics_table_path);
                    return;
                }
            }
        };
        if &diagnostics_table_on_disk != diagnostics_table {
            notify_change(
                diagnostics_table,
                &diagnostics_table_on_disk,
                &diagnostics_table_path,
            )
        }
    }
}
