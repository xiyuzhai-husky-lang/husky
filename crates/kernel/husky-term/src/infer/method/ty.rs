use super::*;

pub(crate) fn ty_method_card(
    db: &dyn TermDb,
    owner_ty: Term,
    ident: Ident,
) -> TermResult<Option<TypeMethodCard>> {
    match owner_ty {
        Term::Literal(_) => unreachable!(),
        Term::Symbol(_) => Ok(None),
        Term::EntityPath(path) => match path {
            TermEntityPath::Form(_) => todo!(),
            TermEntityPath::Trait(_) => todo!(),
            TermEntityPath::TypeOntology(path) => ty_ontology_path_ty_method_card(db, path, ident),
            TermEntityPath::TypeConstructor(_) => todo!(),
        },
        Term::Category(_) => Ok(None),
        Term::Universe(_) => unreachable!(),
        Term::Curry(_) => Ok(None),
        Term::Ritchie(_) => Ok(None),
        Term::Abstraction(_) => unreachable!(),
        Term::Application(raw_ty) => term_application_ty_method_card(db, raw_ty, ident),
        Term::Subentity(_) => todo!(),
        Term::AsTraitSubentity(_) => todo!(),
        Term::TraitConstraint(_) => unreachable!(),
    }
}

pub(crate) fn ty_ontology_path_ty_method_card(
    db: &dyn TermDb,
    path: TypePath,
    ident: Ident,
) -> TermResult<Option<TypeMethodCard>> {
    todo!()
    // let Some(method_raw_ty) = ty_path_ty_method_raw_ty(db, path, ident)? else {
    //     return Ok(None)
    // };
    // Ok(Some(Term::from_raw(
    //     db,
    //     method_raw_ty,
    //     TermTypeExpectation::FinalDestinationEqsSort,
    // )?))
}

#[salsa::tracked(jar = TermJar)]
pub(crate) fn term_application_ty_method_card(
    db: &dyn TermDb,
    raw_ty: TermApplication,
    ident: Ident,
) -> TermResult<Option<TypeMethodCard>> {
    let application_expansion = application_expansion_salsa(db, raw_ty);
    let f = application_expansion.f();
    match f {
        Term::Literal(_) => todo!(),
        Term::Symbol(_) => todo!(),
        Term::EntityPath(path) => match path {
            TermEntityPath::Form(_) => todo!(),
            TermEntityPath::Trait(_) => todo!(),
            TermEntityPath::TypeOntology(path) => ty_ontology_path_application_ty_method_card(
                db,
                path,
                application_expansion.opt_arguments(db).unwrap(),
                ident,
            ),
            TermEntityPath::TypeConstructor(_) => todo!(),
        },
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

fn ty_ontology_path_application_ty_method_card(
    db: &dyn TermDb,
    path: TypePath,
    _arguments: &[Term],
    _ident: Ident,
) -> TermResult<Option<TypeMethodCard>> {
    let ty_method_cards = ty_method_cards(db, path);
    // let decl = match db.ty_decl(path) {
    //     Ok(decl) => decl,
    //     Err(_) => return Err(TermError::TypePathApplicationMethodDeclError),
    // };
    // let _signature = match db.ty_signature(decl) {
    //     Ok(signature) => signature,
    //     Err(_) => return Err(TermError::SignatureError),
    // };
    // let ty_associated_items = db.ty_associated_item_signatures(path);
    todo!()
}

#[salsa::tracked(jar = TermJar, return_ref)]
pub(crate) fn ty_method_cards(
    db: &dyn TermDb,
    path: TypePath,
) -> EntityTreeBundleResult<IdentPairMap<Result<TypeMethodCard, ()>>> {
    let ty_item_decls = db.ty_item_decls(path)?;
    Ok(ty_item_decls
        .iter()
        .copied()
        .filter_map(|(ident, decl)| {
            Some((
                ident,
                match decl {
                    Ok(TypeItemDecl::Method(decl)) => Ok(TypeMethodCard::new(db, decl)),
                    Ok(_) => return None,
                    Err(_) => Err(()),
                },
            ))
        })
        .collect())
}

#[salsa::tracked(db = TermDb, jar = TermJar, constructor = new_inner)]
pub struct TypeMethodCard {
    #[id]
    id: AssociatedItemId,
    pub decl: TypeMethodDecl,
    pub signature: SignatureResult<TypeMethodSignature>,
    pub method_ty: TermResult<Term>,
}

impl TypeMethodCard {
    fn new(db: &dyn TermDb, decl: TypeMethodDecl) -> Self {
        let id = decl.associated_item(db).id(db);
        let signature = todo!();
        let method_ty = todo!();
        Self::new_inner(db, id, decl, signature, method_ty)
    }
}
