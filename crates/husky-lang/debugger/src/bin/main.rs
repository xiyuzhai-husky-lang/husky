use std::fs;

use common::*;
use diagnostic::Diagnostic;
use husky_lang_compile_time::HuskyLangCompileTime;
use husky_lang_debugger::*;

#[tokio::main]
async fn main() {
    let flags = flags::HuskyLangDebuggerCommand::from_env().expect("invalid arguments");
    let mode: Mode = flags.mode.into();
    match mode {
        Mode::Run => run(flags.path.into()).await,
        Mode::TestRuntime => test_runtime(flags.path.into()).await,
        Mode::TestCompileTime => test_diagnostics(flags.path.into()).await,
    }
}

async fn run(path: PathBuf) {
    Debugger::new(|compile_time| init_compile_time_from_path(compile_time, path.into()))
        .serve("localhost:51617")
        .await
        .expect("")
}

async fn test_runtime(dir: PathBuf) {
    assert!(dir.is_dir());
    let package_paths = collect_package_paths(dir);
    println!(
        "\n{}Running{} {} tests on runtime:",
        common::show::CYAN,
        common::show::RESET,
        package_paths.len()
    );
    for package_path in package_paths {
        let error_flag =
            Debugger::new(|compile_time| init_compile_time_from_path(compile_time, package_path))
                .serve_on_error("localhost:51617", 0)
                .await;
        if error_flag {
            return;
        }
    }
}

async fn test_diagnostics(dir: PathBuf) {
    type DiagnosticsTable = HashMap<String, Vec<Diagnostic>>;

    let package_paths = collect_package_paths(dir);
    println!(
        "\n{}Running{} {} tests on compile time:",
        common::show::CYAN,
        common::show::RESET,
        package_paths.len()
    );
    for package_path in package_paths {
        use husky_lang_compile_time::*;
        let mut compile_time = HuskyLangCompileTime::default();
        init_compile_time_from_path(&mut compile_time, package_path.clone());
        println!(
            "\n{}test{} {}",
            common::show::CYAN,
            common::show::RESET,
            package_path.as_os_str().to_str().unwrap(),
        );
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
        compare_diagnostics_tables(diagnostics_table, package_path);
    }

    fn compare_diagnostics_tables(diagnostics_table: DiagnosticsTable, path: PathBuf) {
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
                common::show::CYAN,
                common::show::RESET,
                common::show::GREEN,
                common::show::RESET,
            )
        }
    }
}

fn collect_package_paths(dir: PathBuf) -> Vec<PathBuf> {
    assert!(dir.is_dir());
    let main_path = dir.join("main.hsk");
    if main_path.exists() {
        return vec![dir];
    } else {
        let mut package_paths = vec![];
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let subpath = entry.path();
            if subpath.is_dir() {
                package_paths.extend(collect_package_paths(subpath))
            }
        }
        package_paths
    }
}

fn init_compile_time_from_path(compile_time: &mut HuskyLangCompileTime, path: PathBuf) {
    compile_time.load_package(path)
}

#[derive(Debug)]
pub(crate) enum Mode {
    Run,
    TestCompileTime,
    TestRuntime,
}

impl From<Option<String>> for Mode {
    fn from(opt_str: Option<String>) -> Self {
        if let Some(ref s) = opt_str {
            match s.as_str() {
                "test-runtime" => Mode::TestRuntime,
                "test-compile-time" => Mode::TestCompileTime,
                _ => panic!(),
            }
        } else {
            Mode::Run
        }
    }
}
