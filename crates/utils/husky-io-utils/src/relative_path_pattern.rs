use crate::*;
use composite_pattern::{AtomicPattern, CompositePattern};

pub enum RelativePathPattern {
    BePath { tmpl: RelativePathBuf },
    OfExtension { extension: &'static str },
    NotOfExtension { extension: &'static str },
}

impl AtomicPattern for RelativePathPattern {
    type Input = RelativePath;

    fn contains(&self, path: &RelativePath) -> bool {
        match self {
            RelativePathPattern::OfExtension { extension } => {
                if let Some(path_extension) = path.extension() {
                    path_extension == *extension
                } else {
                    *extension == ""
                }
            }
            RelativePathPattern::NotOfExtension { .. } => todo!(),
            RelativePathPattern::BePath { tmpl } => path == tmpl,
        }
    }
}

impl RelativePathPattern {
    pub fn extension_is_among(
        extensions: impl IntoIterator<Item = &'static str>,
    ) -> CompositePattern<Self> {
        CompositePattern::Or {
            subpatterns: extensions
                .into_iter()
                .map(|extension| RelativePathPattern::OfExtension { extension }.into())
                .collect(),
        }
    }

    pub fn ignore_paths(
        rel_paths: impl IntoIterator<Item = &'static str>,
    ) -> CompositePattern<Self> {
        CompositePattern::NotOr {
            subpatterns: rel_paths
                .into_iter()
                .map(|rel_path| {
                    RelativePathPattern::BePath {
                        tmpl: RelativePathBuf::from(rel_path),
                    }
                    .into()
                })
                .collect(),
        }
    }
}

#[test]
fn test_extension_is_among() {
    let pattern = RelativePathPattern::extension_is_among(["toml", "hsy", "rs"]);
    let path0: RelativePathBuf = "haha.toml".into();
    let path1: RelativePathBuf = "haha.rs".into();
    let path2: RelativePathBuf = "haha.hs".into();
    let path3: RelativePathBuf = "haha".into();
    assert!(pattern.contains(&path0));
    assert!(pattern.contains(&path1));
    assert!(!pattern.contains(&path2));
    assert!(!pattern.contains(&path3));
}

#[test]
fn test_ignore_paths() {
    let pattern = RelativePathPattern::ignore_paths(["haha/target"]);
    let path0: RelativePathBuf = "haha/target".into();
    let path1: RelativePathBuf = "haha/data".into();
    let path2: RelativePathBuf = "target".into();
    assert!(!pattern.contains(&path0));
    assert!(pattern.contains(&path1));
    assert!(pattern.contains(&path2));
}
