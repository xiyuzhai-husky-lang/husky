use crate::*;

pub(crate) fn raw_ty_call_raw_ty(
    db: &dyn RawTypeDb,
    raw_ty_raw_term: ReducedRawTerm,
    toolchain: Toolchain,
    reduced_raw_term_menu: ReducedRawTermMenu,
) -> RawTypeResult<ReducedRawTerm> {
    match raw_ty_raw_term.raw_term() {
        RawTerm::Literal(_) => todo!(),
        RawTerm::Symbol(_) => todo!(),
        RawTerm::EntityPath(path) => match path {
            RawTermEntityPath::Form(_) => todo!(),
            RawTermEntityPath::Trait(_) => todo!(),
            RawTermEntityPath::TypeOntology(_) => todo!(),
            RawTermEntityPath::TypeConstructor(_) => todo!(),
        },
        // EntityPath::Module(_) => todo!(),
        // EntityPath::ModuleItem(path) => match path {
        //     ModuleItemPath::Type(path) => raw_ty_path_raw_ty_call_raw_ty(db, path, toolchain),
        //     ModuleItemPath::Trait(_) => todo!(),
        //     ModuleItemPath::Form(_) => todo!(),
        // },
        // EntityPath::AssociatedItem(_) => todo!(),
        // EntityPath::Variant(_) => todo!(),
        RawTerm::Category(_) => todo!(),
        RawTerm::Universe(_) => todo!(),
        RawTerm::Curry(_) => todo!(),
        RawTerm::Ritchie(_) => todo!(),
        RawTerm::Abstraction(_) => todo!(),
        RawTerm::Application(_) => todo!(),
        RawTerm::Subentity(_) => todo!(),
        RawTerm::AsTraitSubentity(_) => todo!(),
        RawTerm::TraitConstraint(_) => todo!(),
    }
}

#[salsa::tracked(jar = RawTypeJar)]
pub(crate) fn raw_ty_path_raw_ty_call_raw_ty(
    db: &dyn RawTypeDb,
    path: TypePath,
    toolchain: Toolchain,
) -> RawTypeResult<ReducedRawTerm> {
    let decl = match db.ty_decl(path) {
        Ok(decl) => decl,
        Err(_) => return Err(DerivedRawTypeError::DeclError.into()),
    };
    let signature = match db.ty_signature(decl) {
        Ok(signature) => signature,
        Err(_) => return Err(DerivedRawTypeError::SignatureError.into()),
    };
    Err(OriginalRawTypeError::Todo.into())
}
