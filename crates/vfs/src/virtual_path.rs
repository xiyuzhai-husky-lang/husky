use common::*;

use paths::AbsPathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VirtualPath {
    AbsPathBuf(PathBuf),
    NonResidentPath(String),
}

impl VirtualPath {
    pub fn as_path(&self) -> Result<&std::path::Path> {
        todo!()
    }
}

impl From<AbsPathBuf> for VirtualPath {
    fn from(v: AbsPathBuf) -> Self {
        VirtualPath::AbsPathBuf(v.normalize())
    }
}

impl From<PathBuf> for VirtualPath {
    fn from(v: PathBuf) -> Self {
        VirtualPath::AbsPathBuf(v)
    }
}
