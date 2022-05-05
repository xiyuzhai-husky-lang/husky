use crate::*;
use compile_time_db::HuskyLangCompileTime;
use diagnostic::Diagnostic;
use path_utils::collect_all_package_dirs;
use std::path::PathBuf;

#[derive(Debug)]
pub enum Mode {
    Run,
    Test,
}

impl Mode {
    pub async fn run(&self, dir: PathBuf) {
        match self {
            Mode::Run => run(dir).await,
            Mode::Test => test_all_packages_in_dir(dir).await,
        }
    }
}

impl From<Option<String>> for Mode {
    fn from(opt_str: Option<String>) -> Self {
        if let Some(ref s) = opt_str {
            match s.as_str() {
                "test" => Mode::Test,
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

fn init_compile_time_from_dir(compile_time: &mut HuskyLangCompileTime, path: PathBuf) {
    compile_time.load_package(&path)
}

async fn test_all_packages_in_dir(dir: PathBuf) {
    assert!(dir.is_dir());
    let package_paths = collect_all_package_dirs(dir);
    println!(
        "\n{}Running{} tests on {} example packages:",
        print_utils::CYAN,
        print_utils::RESET,
        package_paths.len()
    );

    for package_path in package_paths {
        let mut compile_time = HuskyLangCompileTime::default();
        init_compile_time_from_dir(&mut compile_time, package_path.to_path_buf());
        println!(
            "\n{}test{} {}",
            print_utils::CYAN,
            print_utils::RESET,
            package_path.as_os_str().to_str().unwrap(),
        );
        Debugger::new(|compile_time| init_compile_time_from_dir(compile_time, package_path))
            .serve_on_error("localhost:51617", 0)
            .await;
    }
}

fn report_result_ok() {
    println!(
        "    {}result{}: {}ok{}",
        print_utils::CYAN,
        print_utils::RESET,
        print_utils::GREEN,
        print_utils::RESET,
    )
}
