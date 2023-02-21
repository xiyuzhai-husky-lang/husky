use super::*;
use husky_vfs::Toolchain;

pub(crate) fn term_ty(
    db: &dyn TypeDb,
    reduced_term: ReducedTerm,
    toolchain: Toolchain,
    reduced_term_menu: ReducedTermMenu,
) -> TypeResult<ReducedTerm> {
    match reduced_term.term() {
        Term::Literal(_) => todo!(),
        Term::Symbol(_) => todo!(),
        Term::Entity(path) => entity_path_term_ty(db, path),
        Term::Category(cat) => cat
            .ty()
            .map(Into::into)
            .map(|term| calc_reduced_term(db, term))
            .map_err(|e| OriginalTypeError::Term(e).into()),
        Term::Universe(_) => todo!(),
        Term::Curry(_) => todo!(),
        Term::Ritchie(term) => Ok(match term.ritchie_kind(db) {
            TermRitchieKind::Fp => reduced_term_menu.ty0(),
            TermRitchieKind::Fn | TermRitchieKind::FnMut => reduced_term_menu.trai(),
        }),
        Term::Abstraction(_) => todo!(),
        Term::Application(term) => application_term_ty(db, term),
        Term::Subentity(_) => todo!(),
        Term::AsTraitSubentity(_) => todo!(),
        Term::TraitConstraint(_) => todo!(),
    }
}

pub(crate) fn entity_path_term_ty(db: &dyn TypeDb, path: EntityPath) -> TypeResult<ReducedTerm> {
    match path {
        EntityPath::Module(_) => todo!(),
        EntityPath::ModuleItem(path) => match path {
            ModuleItemPath::Type(path) => ty_path_term_ty(db, path),
            ModuleItemPath::Trait(_) => todo!(),
            ModuleItemPath::Form(_) => todo!(),
        },
        EntityPath::AssociatedItem(_) => todo!(),
        EntityPath::Variant(_) => todo!(),
    }
}

pub(crate) fn ty_path_term_ty(db: &dyn TypeDb, path: TypePath) -> TypeResult<ReducedTerm> {
    let decl = match db.ty_decl(path) {
        Ok(decl) => decl,
        Err(_) => return Err(DerivedTypeError::DeclError.into()),
    };
    let signature = match db.ty_signature(decl) {
        Ok(signature) => signature,
        Err(_) => return Err(DerivedTypeError::SignatureError.into()),
    };
    todo!()
}

#[test]
fn entity_path_path_term_ty_works() {
    let db = DB::default();
    let toolchain = db.dev_toolchain().unwrap();
    let entity_path_menu = db.entity_path_menu(toolchain).unwrap();
    let reduced_term_menu = db.reduced_term_menu(toolchain).unwrap();
    assert_eq!(
        entity_path_term_ty(&db, entity_path_menu.bool().into()),
        Ok(reduced_term_menu.ty0())
    );
    assert_eq!(
        entity_path_term_ty(&db, entity_path_menu.core_ops_add().into()),
        Ok(reduced_term_menu.trai())
    );
    assert_eq!(
        entity_path_term_ty(&db, entity_path_menu.core_ops_add_assign().into()),
        Ok(reduced_term_menu.trai())
    );
    assert_eq!(
        entity_path_term_ty(&db, entity_path_menu.core_ops_bit_and().into()),
        Ok(reduced_term_menu.trai())
    );
    assert_eq!(
        entity_path_term_ty(&db, entity_path_menu.core_ops_bit_and_assign().into()),
        Ok(reduced_term_menu.trai())
    );
    assert_eq!(
        entity_path_term_ty(&db, entity_path_menu.core_ops_bit_or().into()),
        Ok(reduced_term_menu.trai())
    );
    assert_eq!(
        entity_path_term_ty(&db, entity_path_menu.core_ops_bit_or_assign().into()),
        Ok(reduced_term_menu.trai())
    );
    assert_eq!(
        entity_path_term_ty(&db, entity_path_menu.core_ops_bit_xor().into()),
        Ok(reduced_term_menu.trai())
    );
    assert_eq!(
        entity_path_term_ty(&db, entity_path_menu.core_ops_bit_xor_assign().into()),
        Ok(reduced_term_menu.trai())
    );
    assert_eq!(
        entity_path_term_ty(&db, entity_path_menu.core_ops_div().into()),
        Ok(reduced_term_menu.trai())
    );
    assert_eq!(
        entity_path_term_ty(&db, entity_path_menu.core_ops_div_assign().into()),
        Ok(reduced_term_menu.trai())
    );
    assert_eq!(
        entity_path_term_ty(&db, entity_path_menu.core_ops_mul().into()),
        Ok(reduced_term_menu.trai())
    );
    assert_eq!(
        entity_path_term_ty(&db, entity_path_menu.core_ops_mul_assign().into()),
        Ok(reduced_term_menu.trai())
    );
    assert_eq!(
        entity_path_term_ty(&db, entity_path_menu.core_ops_neg().into()),
        Ok(reduced_term_menu.trai())
    );
    assert_eq!(
        entity_path_term_ty(&db, entity_path_menu.core_ops_not().into()),
        Ok(reduced_term_menu.trai())
    );
}

#[salsa::tracked(jar = TypeJar)]
pub(crate) fn application_term_ty(
    db: &dyn TypeDb,
    term: TermApplication,
) -> TypeResult<ReducedTerm> {
    Err(OriginalTypeError::Todo.into())
}
