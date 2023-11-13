use crate::*;
use std::borrow::Borrow;

pub fn diff_file_sync(
    src: impl Borrow<Path>,
    dst: impl Borrow<Path>,
    file_filter_config: FileVisitConfig,
) {
    let config = FileSyncConfig {
        src_root: RelativePathBuf::from_path(src.borrow()).unwrap(),
        dst_root: RelativePathBuf::from_path(dst.borrow()).unwrap(),
        file_visit: file_filter_config,
    };
    diff_file_sync_bfs(src, dst, &config)
}

fn diff_file_sync_bfs(src: impl Borrow<Path>, dst: impl Borrow<Path>, config: &FileSyncConfig) {
    let src = src.borrow();
    let dst = dst.borrow();
    assert!(src.exists());
    if !dst.exists() {
        if src.is_dir() {
            fs::create_dir(dst).unwrap();
            if config.verbose() {
                println!("create dir {:?}", dst)
            }
        }
    }
    if !config.filter_src(&src) {
        return;
    }
    if src.is_file() {
        if dst.is_dir() {
            fs::remove_dir_all(dst).unwrap()
        }
        diff_copy(&src, &dst, config.verbose());
    } else {
        let src_entries: HashMap<String, PathBuf> = fs::read_dir(src)
            .unwrap()
            .map(|entry| {
                let path = entry.unwrap().path();
                (path.file_name().unwrap().to_str().unwrap().to_owned(), path)
            })
            .filter(|(_, path)| config.filter_src(path))
            .collect();
        let dst_entries: HashMap<String, PathBuf> = fs::read_dir(dst)
            .unwrap()
            .map(|entry| {
                let path = entry.unwrap().path();
                (path.file_name().unwrap().to_str().unwrap().to_owned(), path)
            })
            .filter(|(_, path)| config.filter_src(path))
            .collect();
        for (filename, dst_entry) in dst_entries {
            if !config.filter_dst(&dst_entry) {
                continue;
            }
            if !src_entries.contains_key(&filename) {
                if config.verbose() {
                    p!(filename, dst_entry);
                    todo!()
                }
                if dst_entry.is_dir() {
                    std::fs::remove_dir_all(dst_entry).unwrap()
                } else {
                    std::fs::remove_file(dst_entry).unwrap()
                }
            }
        }
        for (filename, src_entry) in src_entries {
            let dst_entry = dst.join(filename);
            diff_file_sync_bfs(src_entry, dst_entry, config)
        }
    }
}
