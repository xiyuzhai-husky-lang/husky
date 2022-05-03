use super::*;
use check_utils::should;
use compile_time_db::*;

pub(super) async fn test_diagnostics(
    package_path: &Path,
    compile_time: &HuskyLangCompileTime,
) -> TestDiagnosticsResult {
    let modules = compile_time.all_modules();
    let mut diagnostics_table = HashMap::<String, Vec<Diagnostic>>::new();
    for module in modules {
        compile_time
            .diagnostics_reserve(module)
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
        &package_path.join("diagnostics_table.txt"),
    );
    if diagnostics_table.len() > 0 {
        TestDiagnosticsResult::HasDiagnostics
    } else {
        TestDiagnosticsResult::NoDiagnostics
    }
}

pub enum TestDiagnosticsResult {
    HasDiagnostics,
    NoDiagnostics,
}
