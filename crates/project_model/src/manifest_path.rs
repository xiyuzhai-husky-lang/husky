use paths::{AbsPath, AbsPathBuf};
use std::{ops, path::Path};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct ManifestPath {
    file: AbsPathBuf,
}

impl ManifestPath {
    // Shadow `parent` from `Deref`.
    pub fn parent(&self) -> &AbsPath {
        self.file.parent().unwrap()
    }
}

impl TryFrom<AbsPathBuf> for ManifestPath {
    type Error = AbsPathBuf;

    fn try_from(file: AbsPathBuf) -> Result<Self, Self::Error> {
        if file.parent().is_none() {
            Err(file)
        } else {
            Ok(ManifestPath { file })
        }
    }
}

impl ops::Deref for ManifestPath {
    type Target = AbsPath;

    fn deref(&self) -> &Self::Target {
        &*self.file
    }
}

impl AsRef<Path> for ManifestPath {
    fn as_ref(&self) -> &Path {
        self.file.as_ref()
    }
}
