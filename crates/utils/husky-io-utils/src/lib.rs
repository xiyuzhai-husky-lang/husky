use husky_print_utils::p;
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

// first read and compare, and then write if different
pub fn diff_write(path: &Path, content: &str) {
    let different = match fs::read_to_string(path) {
        Ok(content_on_disk) => {
            assert!(content_on_disk.len() > 0);
            content != content_on_disk
        }
        Err(_) => true,
    };
    if different {
        match fs::write(path, content) {
            Ok(_) => (),
            Err(e) => {
                p!(path, e);
                todo!()
            }
        }
    }
}

// first read and compare, and then write if different
pub fn diff_copy(src: &Path, dst: &Path) {
    let content = fs::read_to_string(src).unwrap();
    diff_write(dst, &content)
}

pub fn diff_sync(
    src: PathBuf,
    dst: PathBuf,
    extensions: &[&'static str],
    top_excludes: &[&'static str],
    recursive_excludes: &[&'static str],
) {
    assert!(src.exists());
    if !dst.exists() {
        if src.is_dir() {
            fs::create_dir(dst.clone()).unwrap();
        }
    }
    if src.is_file() {
        let src_extension = &src.extension().unwrap().to_str().unwrap();
        if !extensions.contains(&src_extension) {
            panic!("what is this extension {src_extension} at {src:?}")
        }
        if dst.is_dir() {
            fs::remove_dir_all(dst.clone()).unwrap()
        }
        diff_copy(&src, &dst);
    } else {
        let src_entries: HashMap<String, PathBuf> = fs::read_dir(src)
            .unwrap()
            .map(|entry| {
                let path = entry.unwrap().path();
                (path.file_name().unwrap().to_str().unwrap().to_owned(), path)
            })
            .filter(|(filename, _)| {
                !(top_excludes.contains(&(filename as &str))
                    || recursive_excludes.contains(&(filename as &str)))
            })
            .collect();
        let dst_entries: HashMap<String, PathBuf> = fs::read_dir(dst.clone())
            .unwrap()
            .map(|entry| {
                let path = entry.unwrap().path();
                (path.file_name().unwrap().to_str().unwrap().to_owned(), path)
            })
            .filter(|(filename, _)| {
                !(top_excludes.contains(&(filename as &str))
                    || recursive_excludes.contains(&(filename as &str)))
            })
            .collect();
        for (filename, dst_entry) in dst_entries {
            if !src_entries.contains_key(&filename) {
                if dst_entry.is_dir() {
                    std::fs::remove_dir_all(dst_entry).unwrap()
                } else {
                    std::fs::remove_file(dst_entry).unwrap()
                }
            }
        }
        for (filename, src_entry) in src_entries {
            let dst_entry = dst.join(filename);
            diff_sync(src_entry, dst_entry, extensions, &[], recursive_excludes)
        }
    }
}
