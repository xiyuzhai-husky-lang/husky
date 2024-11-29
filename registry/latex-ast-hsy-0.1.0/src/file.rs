use lazy_static::lazy_static;
use std::ops::Deref;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LxFileIdx(u32);

pub struct LxFileStorage {
    files: Vec<String>,
}

lazy_static! {
    pub static ref FILE_STORAGE: LxFileStorage = LxFileStorage::new();
}

impl LxFileStorage {
    pub fn new() -> Self {
        Self {
            files: vec![
                r#"\documentclass{article}
\begin{document}
Hello, world!
\end{document}"#
                    .to_string(),
                r#"\documentclass{article}
\usepackage{amsmath}
\begin{document}
$E = mc^2$
\end{document}"#
                    .to_string(),
            ],
        }
    }

    pub fn get(&self, idx: LxFileIdx) -> Option<&str> {
        self.files.get(idx.0 as usize).map(|s| s.as_str())
    }

    pub fn len(&self) -> usize {
        self.files.len()
    }

    pub fn is_empty(&self) -> bool {
        self.files.is_empty()
    }

    pub fn files(&self) -> &[String] {
        &self.files
    }
}

impl From<u32> for LxFileIdx {
    fn from(idx: u32) -> Self {
        Self(idx)
    }
}

impl Into<u32> for LxFileIdx {
    fn into(self) -> u32 {
        self.0
    }
}

impl std::ops::Index<LxFileIdx> for LxFileStorage {
    type Output = str;

    fn index(&self, idx: LxFileIdx) -> &Self::Output {
        &self.files[idx.0 as usize]
    }
}

#[test]
fn conversion_between_u32_and_file_idx_works() {
    let idx = LxFileIdx(0);
    let u32_val: u32 = idx.into();
    let converted_back: LxFileIdx = u32_val.into();
    assert_eq!(converted_back, idx);
}

#[test]
fn index_operator_works() {
    let storage = LxFileStorage::new();
    let content = &storage[LxFileIdx(0)];
    assert!(content.contains("Hello, world!"));
}
