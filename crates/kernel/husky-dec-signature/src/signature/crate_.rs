pub mod lib;
pub mod main;
pub mod requirements;
pub mod task;

use self::lib::LibCrateDecSignature;
use self::main::MainCrateDecSignature;
use self::requirements::RequirementsCrateDecSignature;
use self::task::TaskCrateDecSignature;
use super::*;
use husky_syn_decl::decl::HasSynNodeDecl;
use husky_vfs::CratePath;

#[enum_class::from_variants]
pub enum CrateDecSignature {
    Lib(LibCrateDecSignature),
    Main(MainCrateDecSignature),
    Requirements(RequirementsCrateDecSignature),
    Task(TaskCrateDecSignature),
}

#[salsa::tracked]
fn crate_dec_signature(db: &::salsa::Db, crate_path: CratePath) -> () {
    let _ = crate_path.syn_decl(db);
    todo!()
}

#[test]
fn crate_dec_signature_works() {
    DB::ast_expect_test_debug_with_db(
        |db, crate_path| crate_dec_signature(db, crate_path),
        &AstTestConfig::new(
            "crate_dec_signature",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::KERNEL,
        ),
    )
}
