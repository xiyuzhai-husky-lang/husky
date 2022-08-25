use husky_path_utils::*;
use husky_print_utils::p;

fn main() {
    let tests_dir: PathBuf = "tests".into();
    for package_dir in collect_package_dirs(&tests_dir) {
        let dirname = package_dir.file_name().unwrap().to_str().unwrap();
        let (intrinsic, last_three_chars) = dirname.split_at(dirname.len() - 3);
        if last_three_chars.chars().all(|c| c.is_digit(10)) {
            let idx = last_three_chars.parse::<u32>().unwrap();
            let new_dirname = format!("{}({})", intrinsic, idx);
            let new_package_dir = package_dir.with_file_name(new_dirname);
            std::fs::rename(package_dir, new_package_dir).unwrap()
        }
    }
}
