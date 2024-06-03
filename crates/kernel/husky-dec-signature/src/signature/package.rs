use super::{form::MajorFormDecTemplate, *};
use husky_entity_path::path::{major_item::MajorItemPath, PrincipalEntityPath};
use husky_entity_tree::{jar::EntityTreeDb, subitem::SubitemPath};
use husky_manifest::synopsis::{package::PackageSynopsis, HasSynopsis};
use husky_vfs::path::package_path::PackagePath;

#[salsa::interned]
pub struct PackageDecSignature {
    pub path: PackagePath,
    #[return_ref]
    pub data: PackageDecSignatureData,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct PackageDecSignatureData {
    pub task_ty_term: Option<DecTerm>,
}

impl PackageDecSignatureData {
    pub fn task_ty_term(&self) -> Option<DecTerm> {
        self.task_ty_term
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
    use husky_manifest::synopsis::HasSynopsis;

    let task_crate_path = match *package_path.synopsis(db)? {
        PackageSynopsis::Lib {
            task_crate_path, ..
        } => task_crate_path,
        PackageSynopsis::Main {
            task_crate_path, ..
        } => Some(task_crate_path),
    };
    let task_ty_term = match task_crate_path {
        Some(task_crate_path) => {
            let task_module_path = task_crate_path.root_module_path(db);
            let coword_menu = coword_menu(db);
            let task_ty_form_path = match db
                .subitem_path(task_module_path.into(), coword_menu.task_ty_ident())
            {
                Ok(subitem_path) => match subitem_path {
                    SubitemPath::Principal(principal_entity_path) => match principal_entity_path {
                        PrincipalEntityPath::Module(_) => todo!(),
                        PrincipalEntityPath::MajorItem(major_item_path) => match major_item_path {
                            MajorItemPath::Type(_) => todo!(),
                            MajorItemPath::Trait(_) => todo!(),
                            MajorItemPath::Form(form_path) => form_path,
                        },
                        PrincipalEntityPath::TypeVariant(_) => todo!(),
                    },
                    SubitemPath::Assoc => todo!(),
                },
                Err(e) => {
                    // it might be because type Task is not pub
                    use husky_print_utils::p;
                    use salsa::DebugWithDb;

                    p!(task_crate_path.debug(db));
                    p!(task_module_path.debug(db));
                    todo!("{e}")
                }
            };
            let MajorFormDecTemplate::TypeAlias(task_ty_alias_dec_template) =
                task_ty_form_path.dec_template(db)?
            else {
                todo!()
            };
            Some(task_ty_alias_dec_template.ty_term(db))
        }
        None => None,
    };
    let data = PackageDecSignatureData { task_ty_term };
    Ok(PackageDecSignature::new(db, package_path, data))
}

#[test]
fn package_dec_signature_works() {
    DB::ast_expect_test_debug_with_db(
        package_dec_signature,
        &AstTestConfig::new(
            "package_dec_signature",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::KERNEL,
        ),
    )
}
