use super::*;
use husky_vfs::PackagePath;

#[salsa::interned]
pub struct PackageDecSignature {
    pub path: PackagePath,
    #[return_ref]
    pub data: PackageDecSignatureData,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct PackageDecSignatureData {
    task_type: Option<DecTerm>,
}

impl PackageDecSignatureData {
    pub fn task_type(&self) -> Option<DecTerm> {
        self.task_type
    }
}

impl HasDecSignature for PackagePath {
    type DecSignature = PackageDecSignature;

    fn dec_signature(self, db: &::salsa::Db) -> DecSignatureResult<Self::DecSignature> {
        package_dec_signature(db, self)
    }
}

#[salsa::tracked]
fn package_dec_signature(
    db: &::salsa::Db,
    package_path: PackagePath,
) -> DecSignatureResult<PackageDecSignature> {
    todo!()
}
