use std::{fs, path::PathBuf};

use crate::*;

use compile_time_db::HuskyLangCompileTime;
use diagnostic::Diagnostic;
use path_utils::collect_package_dirs;

#[derive(Debug)]
pub enum Mode {
    Run,
    TestCompileTime,
    TestRuntime,
}

impl Mode {
    pub async fn run(&self, dir: PathBuf) {
        match self {
            Mode::Run => run(dir).await,
            Mode::TestRuntime => test_runtime(dir).await,
            Mode::TestCompileTime => test_diagnostics(dir).await,
        }
    }
}

impl From<Option<String>> for Mode {
    fn from(opt_str: Option<String>) -> Self {
        if let Some(ref s) = opt_str {
            match s.as_str() {
                "test-runtime" => Mode::TestRuntime,
                "test-compile-time" => Mode::TestCompileTime,
                "run" => Mode::Run,
                _ => panic!(),
            }
        } else {
            Mode::Run
        }
    }
}

async fn run(path: PathBuf) {
    Debugger::new(|compile_time| init_compile_time_from_dir(compile_time, path.into()))
        .serve("localhost:51617")
        .await
        .expect("")
}

async fn test_runtime(dir: PathBuf) {
    assert!(dir.is_dir());
    let package_paths = collect_package_dirs(dir);
    println!(
        "\n{}Running{} {} tests on runtime:",
        print_utils::CYAN,
        print_utils::RESET,
        package_paths.len()
    );
    for package_path in package_paths {
        let error_flag =
            Debugger::new(|compile_time| init_compile_time_from_dir(compile_time, package_path))
                .serve_on_error("localhost:51617", 0)
                .await;
        if error_flag {
            return;
        }
    }
}

async fn test_diagnostics(dir: PathBuf) {
    type DiagnosticsTable = HashMap<String, Vec<Diagnostic>>;

    let package_paths = collect_package_dirs(dir);
    println!(
        "\n{}Running{} {} tests on compile time:",
        print_utils::CYAN,
        print_utils::RESET,
        package_paths.len()
    );
    for package_path in package_paths {
        use compile_time_db::*;
        let mut compile_time = HuskyLangCompileTime::default();
        init_compile_time_from_dir(&mut compile_time, package_path.clone());
        println!(
            "\n{}test{} {}",
            print_utils::CYAN,
            print_utils::RESET,
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
                print_utils::CYAN,
                print_utils::RESET,
                print_utils::GREEN,
                print_utils::RESET,
            )
        }
    }
}

fn init_compile_time_from_dir(compile_time: &mut HuskyLangCompileTime, path: PathBuf) {
    compile_time.load_package(path)
}
