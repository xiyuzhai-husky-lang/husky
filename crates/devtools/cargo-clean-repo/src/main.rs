use husky_path_utils::{clear_directory, collect_rust_package_dirs, find_paths, Path};
use husky_print_utils::p;
use std::path::PathBuf;

fn main() {
    // make sure that this is the repository  https://github.com/xiyuzhai-husky-lang/husky
    assert!(PathBuf::from("Cargo.toml").exists());
    assert!(PathBuf::from("crates").exists());
    assert!(PathBuf::from("rust-toolchain").exists());
    assert!(PathBuf::from("husky-toolchain.toml").exists());
    assert!(PathBuf::from(".corgi/config.toml").exists());
    remove_folder_in_tests("try/try");
    // clean_expect_files();
    // clean_tests()
    // restructure()
}

fn clean_expect_files() {
    for dir in collect_rust_package_dirs(".") {
        assert!(dir.join("Cargo.toml").exists());
        let expect_files_dir = dir.join("expect-files");
        if expect_files_dir.exists() {
            assert!(expect_files_dir.is_dir());
        }
        clear_directory(&expect_files_dir);
    }
}

fn remove_folder_in_tests(ends_with: &str) {
    let collect_paths = find_paths(&PathBuf::from("tests"));
    for path in collect_paths {
        if path.ends_with(ends_with) {
            std::fs::remove_dir_all(path).unwrap()
        }
    }
}

fn restructure() {
    fn corgi_toml(package_name: &str) -> String {
        format!(
            r#"[package]
name = "{package_name}""#
        )
    }

    let collect_paths = find_paths(&PathBuf::from("tests"));
    for path in collect_paths {
        if path.join("main.hsy").exists() {
            let package_name = path.file_name().unwrap().to_str().unwrap().to_owned();
            if package_name == "src" {
                continue;
            }
            let mut jobs = vec![];
            for entry in std::fs::read_dir(path.clone()).unwrap() {
                let subpath = entry.unwrap().path();
                if !subpath.ends_with("rust") && !subpath.ends_with("src") {
                    let newpath = subpath
                        .parent()
                        .unwrap()
                        .join("src")
                        .join(subpath.file_name().unwrap());
                    jobs.push((subpath, newpath))
                }
            }
            std::fs::create_dir_all(path.join("src")).unwrap();
            for (subpath, newpath) in jobs {
                p!(subpath, newpath);
                std::fs::rename(subpath, newpath).unwrap()
            }
            std::fs::write(path.join("Corgi.toml"), corgi_toml(&package_name)).unwrap()
        }
    }
}
