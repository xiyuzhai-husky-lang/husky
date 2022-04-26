mod test_compile_time;
mod test_runtime;

use crate::*;
use compile_time_db::HuskyLangCompileTime;
use diagnostic::Diagnostic;
use path_utils::collect_pack_dirs;
use std::{fs, path::PathBuf};
use test_compile_time::*;
use test_runtime::*;

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
            Mode::TestCompileTime => test_compile_time(dir).await,
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

fn init_compile_time_from_dir(compile_time: &mut HuskyLangCompileTime, path: PathBuf) {
    compile_time.load_pack(path)
}
