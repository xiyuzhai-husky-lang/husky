pub mod lib;
pub mod main;
pub mod requirements;
pub mod task;

use self::lib::LibCrateEthSignature;
use self::main::MainCrateEthSignature;
use self::requirements::RequirementsCrateEthSignature;
use self::task::TaskCrateEthSignature;
use super::*;
use husky_dec_signature::signature::crate_::CrateDecSignature;
use husky_vfs::{CrateKind, CratePath};

#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CrateEthSignature {
    Lib(LibCrateEthSignature),
    Main(MainCrateEthSignature),
    Requirements(RequirementsCrateEthSignature),
    Task(TaskCrateEthSignature),
}

impl HasEthSignature for CratePath {
    type EthSignature = CrateEthSignature;

    fn eth_signature(self, db: &salsa::Db) -> EthSignatureResult<Self::EthSignature> {
        crate_eth_signature(db, self)
    }
}

#[salsa::tracked]
fn crate_eth_signature(
    db: &::salsa::Db,
    crate_path: CratePath,
) -> EthSignatureResult<CrateEthSignature> {
    let dec_signature = crate_path.dec_signature(db)?;
    Ok(match dec_signature {
        CrateDecSignature::Lib(dec_signature) => {
            LibCrateEthSignature::from_dec(crate_path, dec_signature, db)?.into()
        }
        CrateDecSignature::Main(dec_signature) => {
            MainCrateEthSignature::from_dec(crate_path, dec_signature, db)?.into()
        }
        CrateDecSignature::Requirements(dec_signature) => {
            RequirementsCrateEthSignature::from_dec(crate_path, dec_signature, db)?.into()
        }
        CrateDecSignature::Task(dec_signature) => {
            TaskCrateEthSignature::from_dec(crate_path, dec_signature, db)?.into()
        }
    })
}

#[test]
fn crate_dec_signature_works() {
    DB::ast_expect_test_debug_with_db(
        |db, crate_path| crate_eth_signature(db, crate_path),
        &AstTestConfig::new(
            "crate_dec_signature",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::KERNEL,
        ),
    )
}
