pub mod lib;
pub mod main;
pub mod requirements;
pub mod task;

use self::lib::LibCrateDecSignature;
use self::main::MainCrateDecSignature;
use self::requirements::RequirementsCrateDecSignature;
use self::task::TaskCrateDecSignature;
use super::*;
use husky_syn_decl::decl::crate_::CrateSynDecl;
use husky_vfs::{CrateKind, CratePath};

#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CrateDecSignature {
    Lib(LibCrateDecSignature),
    Main(MainCrateDecSignature),
    Requirements(RequirementsCrateDecSignature),
    Task(TaskCrateDecSignature),
}

impl HasDecSignature for CratePath {
    type DecSignature = CrateDecSignature;

    fn dec_signature(self, db: &salsa::Db) -> DecSignatureResult<Self::DecSignature> {
        crate_dec_signature(db, self)
    }
}

#[salsa::tracked]
fn crate_dec_signature(
    db: &::salsa::Db,
    crate_path: CratePath,
) -> DecSignatureResult<CrateDecSignature> {
    let Some(syn_decl) = crate_path.syn_decl(db)? else {
        return Ok(match crate_path.kind(db) {
            CrateKind::Lib => LibCrateDecSignature::from_decl(crate_path, None, db)?.into(),
            CrateKind::Main => MainCrateDecSignature::from_decl(crate_path, None, db)?.into(),
            CrateKind::Requirements => {
                RequirementsCrateDecSignature::from_decl(crate_path, None, db)?.into()
            }
            CrateKind::Task => TaskCrateDecSignature::from_decl(crate_path, None, db)?.into(),
            CrateKind::Bin(_) => todo!(),
            CrateKind::IntegratedTest(_) => todo!(),
            CrateKind::Example => todo!(),
        });
    };
    Ok(match syn_decl {
        CrateSynDecl::Lib(syn_decl) => {
            LibCrateDecSignature::from_decl(crate_path, syn_decl, db)?.into()
        }
        CrateSynDecl::Main(syn_decl) => {
            MainCrateDecSignature::from_decl(crate_path, syn_decl, db)?.into()
        }
        CrateSynDecl::Requirements(syn_decl) => {
            RequirementsCrateDecSignature::from_decl(crate_path, syn_decl, db)?.into()
        }
        CrateSynDecl::Task(syn_decl) => {
            TaskCrateDecSignature::from_decl(crate_path, syn_decl, db)?.into()
        }
    })
}

#[test]
fn crate_dec_signature_works() {
    DB::ast_expect_test_debug_with_db(
        crate_dec_signature,
        &AstTestConfig::new(
            "crate_dec_signature",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::KERNEL,
        ),
    )
}
