use super::*;
use either::*;

#[salsa::interned(jar = HirTypeJar)]
pub struct HirTypePathLeading {
    pub ty_path: TypePath,
    /// phantom arguments are ignored
    #[return_ref]
    pub template_arguments: HirTemplateArguments,
}

impl HirTypePathLeading {
    pub fn is_copyable_obviously(self, db: &::salsa::Db) -> bool {
        hir_ty_path_leading_is_copyable_obviously(db, self)
    }
}

#[deprecated(note = "ad hoc implementation")]
#[salsa::tracked(jar = HirTypeJar)]
fn hir_ty_path_leading_is_copyable_obviously(db: &::salsa::Db, hir_ty: HirTypePathLeading) -> bool {
    match hir_ty.ty_path(db).refine(db) {
        Left(prelude_ty_path) => match prelude_ty_path {
            PreludeTypePath::Basic(_) => true,
            PreludeTypePath::Num(_) => true,
            PreludeTypePath::Indirection(prelude_indirection_ty_path) => {
                match prelude_indirection_ty_path {
                    PreludeIndirectionTypePath::Ref => true,
                    PreludeIndirectionTypePath::RefMut => false,
                    PreludeIndirectionTypePath::Leash => true,
                    PreludeIndirectionTypePath::At => todo!(),
                }
            }
            PreludeTypePath::Nat => unreachable!(),
            PreludeTypePath::Lifetime => unreachable!(),
            PreludeTypePath::Module => unreachable!(),
            PreludeTypePath::Trait => unreachable!(),
            PreludeTypePath::StringLiteral => true,
            PreludeTypePath::Str => false,
            PreludeTypePath::Option => {
                debug_assert_eq!(hir_ty.template_arguments(db).len(), 1);
                let HirTemplateArgument::Type(hir_ty_t) = hir_ty.template_arguments(db)[0] else {
                    unreachable!()
                };
                hir_ty_t.is_copyable_obviously(db)
            }
            PreludeTypePath::Result => {
                debug_assert_eq!(hir_ty.template_arguments(db).len(), 2);
                let HirTemplateArgument::Type(hir_ty_t) = hir_ty.template_arguments(db)[0] else {
                    unreachable!()
                };
                let HirTemplateArgument::Type(hir_ty_e) = hir_ty.template_arguments(db)[1] else {
                    unreachable!()
                };
                hir_ty_t.is_copyable_obviously(db) && hir_ty_e.is_copyable_obviously(db)
            }
            _ => false,
        },
        Right(_) => false,
    }
}
