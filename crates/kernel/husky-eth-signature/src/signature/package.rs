use super::*;
use husky_dec_signature::signature::package::{PackageDecSignature, PackageDecSignatureData};
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

impl IsPackageEthSignatureData for PackageEthSignatureData {
    fn task_ty(&self) -> Option<EthTerm> {
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
    let dec_signature = package_path.dec_signature(db)?;
    PackageEthSignature::from_dec(package_path, dec_signature, db)
}

impl PackageEthSignature {
    fn from_dec(
        package_path: PackagePath,
        dec_signature: PackageDecSignature,
        db: &::salsa::Db,
    ) -> EthSignatureResult<Self> {
        let PackageDecSignatureData { task_ty_term } = *dec_signature.data(db);
        Ok(Self::new(
            db,
            package_path,
            PackageEthSignatureData {
                task_type: match task_ty_term {
                    Some(task_ty_term) => Some(EthTerm::ty_from_dec(db, task_ty_term)?),
                    None => None,
                },
            },
        ))
    }
}

#[test]
fn package_eth_signature_works() {
    DB::ast_expect_test_debug_with_db(
        package_eth_signature,
        &AstTestConfig::new(
            "package_eth_signature",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::KERNEL,
        ),
    )
}
