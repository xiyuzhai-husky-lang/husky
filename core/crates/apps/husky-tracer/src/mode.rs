use crate::*;
use path_utils::collect_all_package_dirs;
use static_root::static_root_defn;
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
    HuskyTracer::new(|compile_time| init_compile_time_from_dir(compile_time, path.into()))
        .serve("localhost:51617")
        .await
        .expect("")
}

fn init_compile_time_from_dir(compile_time: &mut HuskyCompileTime, dir: PathBuf) {
    compile_time.set_main_package(&dir);
    compile_time.load_package(&dir)
}

async fn test_all_packages_in_dir(dir: PathBuf) {
    assert!(dir.is_dir());
    let package_dirs = collect_all_package_dirs(dir);
    println!(
        "\n{}Running{} tests on {} example packages:",
        print_utils::CYAN,
        print_utils::RESET,
        package_dirs.len()
    );

    for package_dir in package_dirs {
        let mut compile_time = HuskyCompileTime::new(static_root_defn);
        init_compile_time_from_dir(&mut compile_time, package_dir.to_path_buf());
        println!(
            "\n{}test{} {}",
            print_utils::CYAN,
            print_utils::RESET,
            package_dir.as_os_str().to_str().unwrap(),
        );
        match HuskyTracer::new(|compile_time| init_compile_time_from_dir(compile_time, package_dir))
            .serve_on_error("localhost:51617", SampleId(0))
            .await
        {
            TestResult::Success => finalize_success(),
            TestResult::Failed => finalize_failure(),
        }
    }
}

fn finalize_success() {
    println!(
        "    {}result{}: {}success{}",
        print_utils::CYAN,
        print_utils::RESET,
        print_utils::GREEN,
        print_utils::RESET,
    )
}

fn finalize_failure() {
    println!(
        "    {}result{}: {}failure{}",
        print_utils::CYAN,
        print_utils::RESET,
        print_utils::RED,
        print_utils::RESET,
    )
}
