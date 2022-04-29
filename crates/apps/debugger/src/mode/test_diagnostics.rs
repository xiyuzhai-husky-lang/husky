use super::*;
use compile_time_db::*;

pub(super) async fn test_diagnostics(
    pack_path: &Path,
    compile_time: &HuskyLangCompileTime,
) -> bool {
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
    compare_saved_data(
        &diagnostics_table,
        &pack_path.join("diagnostics_table.json"),
    );
    return diagnostics_table.len() > 0;
}
