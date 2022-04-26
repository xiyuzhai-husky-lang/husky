use super::*;

pub(super) async fn test_runtime(dir: PathBuf) {
    assert!(dir.is_dir());
    let pack_paths = collect_pack_dirs(dir);
    println!(
        "\n{}Running{} {} tests on runtime:",
        print_utils::CYAN,
        print_utils::RESET,
        pack_paths.len()
    );
    for pack_path in pack_paths {
        let error_flag =
            Debugger::new(|compile_time| init_compile_time_from_dir(compile_time, pack_path))
                .serve_on_error("localhost:51617", 0)
                .await;
        if error_flag {
            return;
        }
    }
}
