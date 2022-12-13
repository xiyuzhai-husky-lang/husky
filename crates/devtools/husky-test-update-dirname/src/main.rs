use husky_control_flow_utils::loop_require;
use husky_path_utils::*;

fn main() {
    let tests_dir: PathBuf = "tests".into();
    for package_dir in collect_package_dirs_deprecated(&tests_dir) {
        let dirname = package_dir.file_name().unwrap().to_str().unwrap();
        let splits: Vec<&str> = dirname.split('(').collect();
        loop_require!(splits.len() == 2);
        let intrinsic = splits[0];
        let digit_str = splits[1].split(')').next().unwrap();
        assert!(digit_str.chars().all(|c| c.is_digit(10)));
        let idx = digit_str.parse::<u32>().unwrap();
        let new_dirname = format!("{}--{}", intrinsic, idx);
        let new_package_dir = package_dir.with_file_name(new_dirname);
        std::fs::rename(package_dir, new_package_dir).unwrap();
    }
}
