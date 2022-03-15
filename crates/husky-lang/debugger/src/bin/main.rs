use std::fs;

use common::PathBuf;
use husky_lang_compile_time::HuskyLangCompileTime;
use husky_lang_debugger::*;

#[tokio::main]
async fn main() {
    let flags = flags::HuskyLangDebuggerCommand::from_env().expect("invalid arguments");
    let mode: Mode = flags.mode.into();
    match mode {
        Mode::Run => run(flags.path.into()).await,
        Mode::Test => test(flags.path.into()).await,
    }
}

async fn run(path: PathBuf) {
    Debugger::new(|compile_time| init_compile_time_from_path(compile_time, path.into()))
        .serve("localhost:51617")
        .await
        .expect("")
}

async fn test(path: PathBuf) {
    assert!(path.is_dir());
    let package_paths = collect_package_paths(path);
    println!(
        "\n{}Running{} {} tests:",
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

fn collect_package_paths(path: PathBuf) -> Vec<PathBuf> {
    assert!(path.is_dir());
    let main_path = path.join("main.hsk");
    if main_path.exists() {
        return vec![path];
    } else {
        let mut package_paths = vec![];
        for entry in fs::read_dir(path).unwrap() {
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

pub(crate) enum Mode {
    Run,
    Test,
}

impl From<Option<String>> for Mode {
    fn from(opt_str: Option<String>) -> Self {
        if let Some(ref s) = opt_str {
            match s.as_str() {
                "test" => Mode::Test,
                _ => panic!(),
            }
        } else {
            Mode::Run
        }
    }
}
