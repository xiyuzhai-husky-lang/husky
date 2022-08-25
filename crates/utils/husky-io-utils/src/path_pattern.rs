use std::path::{Path, PathBuf};

use composite_pattern::{AtomicPattern, CompositePattern};

pub enum PathPattern {
    BePath { path_tmpl: PathBuf },
    OfExtension { extension: &'static str },
    NotOfExtension { extension: &'static str },
}

impl AtomicPattern for PathPattern {
    type Input = Path;

    fn contains(&self, path: &Path) -> bool {
        match self {
            PathPattern::OfExtension { extension } => {
                if let Some(path_extension) = path.extension() {
                    if let Some(path_extension) = path_extension.to_str() {
                        path_extension == *extension
                    } else {
                        false
                    }
                } else {
                    *extension == ""
                }
            }
            PathPattern::NotOfExtension { extension } => todo!(),
            PathPattern::BePath { path_tmpl } => path == (&path_tmpl as &Path),
        }
    }
}

impl PathPattern {
    pub fn extension_is_among(
        extensions: impl IntoIterator<Item = &'static str>,
    ) -> CompositePattern<Self> {
        CompositePattern::Any {
            subpatterns: extensions
                .into_iter()
                .map(|extension| PathPattern::OfExtension { extension }.into())
                .collect(),
        }
    }

    pub fn ignore_paths(
        root: &Path,
        rel_paths: impl IntoIterator<Item = &'static str>,
    ) -> CompositePattern<Self> {
        CompositePattern::NotAny {
            subpatterns: rel_paths
                .into_iter()
                .map(|rel_path| {
                    PathPattern::BePath {
                        path_tmpl: root.join(rel_path),
                    }
                    .into()
                })
                .collect(),
        }
    }
}

#[test]
fn test_extension_is_among() {
    let pattern = PathPattern::extension_is_among(["toml", "hsk", "rs"]);
    let path0: PathBuf = "haha.toml".into();
    let path1: PathBuf = "haha.rs".into();
    let path2: PathBuf = "haha.hs".into();
    let path3: PathBuf = "haha".into();
    assert!(pattern.contains(&path0));
    assert!(pattern.contains(&path1));
    assert!(!pattern.contains(&path2));
    assert!(!pattern.contains(&path3));
}

#[test]
fn test_ignore_paths() {
    let root: PathBuf = "haha".into();
    let pattern = PathPattern::ignore_paths(&root, ["target"]);
    let path0: PathBuf = "haha/target".into();
    let path1: PathBuf = "haha/data".into();
    let path2: PathBuf = "target".into();
    assert!(!pattern.contains(&path0));
    assert!(pattern.contains(&path1));
    assert!(pattern.contains(&path2));
}
