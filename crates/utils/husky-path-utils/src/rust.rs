use crate::*;

pub fn cargo_manifest_dir() -> Result<PathBuf, std::env::VarError> {
    std::env::var("CARGO_MANIFEST_DIR").map(|s| s.into())
}

pub fn husky_cargo_workspace_manifest_dir() -> PathBuf {
    let base_dir = cargo_manifest_dir().unwrap();
    let mut dir: &Path = &base_dir;
    let mut last_manifest_dir: &Path = &dir;
    loop {
        let Some(parent_dir) = dir.parent() else {
            return last_manifest_dir.to_owned();
        };
        dir = parent_dir;
        let cargo_manifest_path = &dir.join("Cargo.toml");
        if cargo_manifest_path.exists() {
            assert!(cargo_manifest_path.is_file());
            last_manifest_dir = dir;
        };
    }
}

/// collect all cargo manifest directories under  `dir`, `dir` included
///
/// Here a diretory is seen as a cargo manifest directory if it has a valid file named "Cargo.toml" and a valid folder named "src" under it
pub fn collect_cargo_manifest_dirs(dir: impl AsRef<Path>) -> Vec<PathBuf> {
    let dir = dir.as_ref();
    should_satisfy!(&dir, |dir: &Path| dir.is_dir());
    let mut pack_paths = vec![];
    collect_cargo_manifest_dirs_aux(dir, &mut pack_paths);
    pack_paths.sort();
    pack_paths
}

fn collect_cargo_manifest_dirs_aux(dir: impl AsRef<Path>, pack_paths: &mut Vec<PathBuf>) {
    let dir = dir.as_ref();
    let manifest_path = dir.join("Cargo.toml");
    for entry in std::fs::read_dir(&dir).unwrap() {
        let entry = entry.unwrap();
        let subpath = entry.path();
        if !subpath.ends_with("target") && subpath.is_dir() {
            collect_cargo_manifest_dirs_aux(&subpath, pack_paths)
        }
    }
    if manifest_path.exists() {
        assert!(manifest_path.is_file(), "expect manifest path to be file");
        let src_path = dir.join("src");
        if src_path.exists() {
            assert!(src_path.is_dir(), "expect src path to be dir");
            pack_paths.push(dir.to_owned())
        }
    }
}

pub fn collect_rust_source_paths_with(
    dir: impl AsRef<Path>,
    f: impl Fn(&Path) -> bool,
) -> Vec<PathBuf> {
    let mut paths: Vec<PathBuf> = vec![];
    collect_cargo_manifest_dirs(dir)
        .into_iter()
        .for_each(|dir| collect_rust_source_paths_in_cargo_manifest_dir_with(&dir, &f, &mut paths));
    paths
}

fn collect_rust_source_paths_in_cargo_manifest_dir_with(
    cargo_manifest_dir: &Path,
    f: &impl Fn(&Path) -> bool,
    paths: &mut Vec<PathBuf>,
) {
    let parent_dir = cargo_manifest_dir.join("src");
    debug_assert!(parent_dir.is_dir());
    debug_assert!(parent_dir.exists());
    collect_rust_source_paths_in_cargo_manifest_dir_with_aux(&parent_dir, f, paths)
}

fn collect_rust_source_paths_in_cargo_manifest_dir_with_aux(
    parent_dir: &Path,
    f: &impl Fn(&Path) -> bool,
    paths: &mut Vec<PathBuf>,
) {
    for entry in std::fs::read_dir(parent_dir).unwrap() {
        let entry = entry.unwrap();
        let subpath = entry.path();
        debug_assert!(subpath.exists());
        if subpath.is_dir() {
            collect_rust_source_paths_in_cargo_manifest_dir_with_aux(&subpath, f, paths)
        } else {
            debug_assert!(subpath.is_file());
            if f(&subpath) {
                paths.push(subpath)
            }
        }
    }
}
