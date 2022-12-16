use husky_package_path::PackagePath;
use husky_word::Identifier;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct CratePath {
    package: PackagePath,
    crate_kind: CrateKind,
}

impl CratePath {
    pub(crate) fn new(package: PackagePath, kind: CrateKind) -> Self {
        Self {
            package,
            crate_kind: kind,
        }
    }

    pub fn package(&self) -> PackagePath {
        self.package
    }

    pub fn crate_kind(&self) -> CrateKind {
        self.crate_kind
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum CrateKind {
    Library,
    Main,
    Binary(Identifier),
}

impl CrateKind {
    pub fn path(&self) -> &'static str {
        match self {
            CrateKind::Library => "src/lib.hsy",
            CrateKind::Main => "src/main.hsy",
            CrateKind::Binary(_) => todo!(),
        }
    }
}
