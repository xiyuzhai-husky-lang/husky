use crate::*;

pub(crate) fn ty_call_ty(
    db: &dyn TypeDb,
    ty_term: ReducedTerm,
    toolchain: Toolchain,
    reduced_term_menu: ReducedTermMenu,
) -> TypeResult<ReducedTerm> {
    match ty_term.term() {
        Term::Literal(_) => todo!(),
        Term::Symbol(_) => todo!(),
        Term::EntityPath(path) => match path {
            TermEntityPath::Form(_) => todo!(),
            TermEntityPath::Trait(_) => todo!(),
            TermEntityPath::TypeOntology(_) => todo!(),
            TermEntityPath::TypeConstructor(_) => todo!(),
        },
        // EntityPath::Module(_) => todo!(),
        // EntityPath::ModuleItem(path) => match path {
        //     ModuleItemPath::Type(path) => ty_path_ty_call_ty(db, path, toolchain),
        //     ModuleItemPath::Trait(_) => todo!(),
        //     ModuleItemPath::Form(_) => todo!(),
        // },
        // EntityPath::AssociatedItem(_) => todo!(),
        // EntityPath::Variant(_) => todo!(),
        Term::Category(_) => todo!(),
        Term::Universe(_) => todo!(),
        Term::Curry(_) => todo!(),
        Term::Ritchie(_) => todo!(),
        Term::Abstraction(_) => todo!(),
        Term::Application(_) => todo!(),
        Term::Subentity(_) => todo!(),
        Term::AsTraitSubentity(_) => todo!(),
        Term::TraitConstraint(_) => todo!(),
    }
}

#[salsa::tracked(jar = TypeJar)]
pub(crate) fn ty_path_ty_call_ty(
    db: &dyn TypeDb,
    path: TypePath,
    toolchain: Toolchain,
) -> TypeResult<ReducedTerm> {
    todo!()
}
