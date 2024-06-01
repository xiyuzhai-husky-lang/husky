use super::*;
use husky_vfs::path::package_path::PackagePath;

#[salsa::interned]
pub struct PackageEthSignature {
    pub path: PackagePath,
    #[return_ref]
    pub data: PackageEthSignatureData,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct PackageEthSignatureData {
    task_type: Option<EthTerm>,
}

impl PackageEthSignatureData {
    pub fn task_type(&self) -> Option<EthTerm> {
        self.task_type
    }
}

impl HasEthSignature for PackagePath {
    type EthSignature = PackageEthSignature;

    fn eth_signature(self, db: &::salsa::Db) -> EthSignatureResult<Self::EthSignature> {
        package_eth_signature(db, self)
    }
}

#[salsa::tracked]
fn package_eth_signature(
    db: &::salsa::Db,
    package_path: PackagePath,
) -> EthSignatureResult<PackageEthSignature> {
    todo!()
}
