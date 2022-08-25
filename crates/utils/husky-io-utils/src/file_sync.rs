use std::borrow::Borrow;

use crate::*;

pub fn diff_file_sync(src: impl Borrow<Path>, dst: impl Borrow<Path>, config: &FileVisitConfig) {
    let src = src.borrow();
    let dst = dst.borrow();
    assert!(src.exists());
    if !dst.exists() {
        if src.is_dir() {
            fs::create_dir(dst.clone()).unwrap();
        }
    }
    if !config.filter(&src) {
        return;
    }
    if src.is_file() {
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
            .filter(|(_, path)| config.filter(path))
            .collect();
        let dst_entries: HashMap<String, PathBuf> = fs::read_dir(dst.clone())
            .unwrap()
            .map(|entry| {
                let path = entry.unwrap().path();
                (path.file_name().unwrap().to_str().unwrap().to_owned(), path)
            })
            .filter(|(_, path)| config.filter(path))
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
            diff_file_sync(src_entry, dst_entry, config)
        }
    }
}
