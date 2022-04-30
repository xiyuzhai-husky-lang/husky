mod compare;
mod test_diagnostics;
mod test_semantic_tokens;

use crate::*;
use compare::*;
use compile_time_db::HuskyLangCompileTime;
use diagnostic::Diagnostic;
use path_utils::collect_pack_dirs;
use std::path::PathBuf;
use test_diagnostics::*;
use test_semantic_tokens::*;

#[derive(Debug)]
pub enum Mode {
    Run,
    Test,
}

impl Mode {
    pub async fn run(&self, dir: PathBuf) {
        match self {
            Mode::Run => run(dir).await,
            Mode::Test => test(dir).await,
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
    compile_time.load_pack(path)
}

async fn test(dir: PathBuf) {
    assert!(dir.is_dir());
    let pack_paths = collect_pack_dirs(dir);
    println!(
        "\n{}Running{} tests on {} example packages:",
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
        if test_diagnostics(&pack_path, &compile_time).await {
            report_result_ok();
            continue;
        }
        let error_flag =
            Debugger::new(|compile_time| init_compile_time_from_dir(compile_time, pack_path))
                .serve_on_error("localhost:51617", 0)
                .await;
        if error_flag {
            return;
        }
        report_result_ok();
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
