use super::*;
use check_utils::should;
use compile_time_db::*;

pub(super) async fn test_diagnostics(
    package_path: &Path,
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
    if diagnostics_table.len() == 0 {
        match compile_time.package(compile_time.intern_file(package_path.join("main.hsk"))) {
            Ok(_) => (),
            Err(e) => should!(diagnostics_table
                .insert("package".into(), vec![e.into()])
                .is_none()),
        }
    }
    compare_saved_data(
        &diagnostics_table,
        &package_path.join("diagnostics_table.json"),
    );
    return diagnostics_table.len() > 0;
}
