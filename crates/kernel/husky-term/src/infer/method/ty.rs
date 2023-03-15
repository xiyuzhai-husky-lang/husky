use husky_entity_tree::EntityTreeBundleResultRef;
use husky_signature::{ty_method_signature, RegularParameterSignature};
use vec_like::VecMapGetEntry;

use super::*;

pub(crate) fn ty_method_card(
    db: &dyn TermDb,
    owner_ty: Term,
    ident: Ident,
) -> TermResult<Option<TypeMethodCard>> {
    assert!(owner_ty.is_reduced(db));
    // using the fact that owner_ty is reduced
    match owner_ty {
        Term::EntityPath(TermEntityPath::TypeOntology(path)) => {
            ty_ontology_path_ty_method_card(db, path, ident)
        }
        Term::Application(raw_ty) => term_application_ty_method_card(db, raw_ty, ident),
        _ => Ok(None),
    }
}

pub(crate) fn ty_ontology_path_ty_method_card(
    db: &dyn TermDb,
    path: TypePath,
    ident: Ident,
) -> TermResult<Option<TypeMethodCard>> {
    let ty_method_cards = ty_path_ty_method_cards(db, path)?;
    let Some(entry) = ty_method_cards.get_entry(ident) else {
        return Ok(None)
    };
    let Ok(ty_method_card) = entry.1 else {
        todo!()
    };
    Ok(Some(ty_method_card))
}

#[salsa::tracked(jar = TermJar)]
pub(crate) fn term_application_ty_method_card(
    db: &dyn TermDb,
    raw_ty: TermApplication,
    ident: Ident,
) -> TermResult<Option<TypeMethodCard>> {
    let application_expansion = application_expansion_salsa(db, raw_ty);
    let function = application_expansion.function();
    match function {
        TermFunctionReduced::TypeOntology(path) => ty_ontology_path_application_ty_method_card(
            db,
            path,
            application_expansion.opt_arguments(db).unwrap(),
            ident,
        ),
    }
}

fn ty_ontology_path_application_ty_method_card(
    db: &dyn TermDb,
    path: TypePath,
    _arguments: &[Term],
    ident: Ident,
) -> TermResult<Option<TypeMethodCard>> {
    let ty_method_cards = ty_path_ty_method_cards(db, path)?;
    let Some(entry) = ty_method_cards.get_entry(ident) else {
        return Ok(None)
    };
    let Ok(ty_method_card) = entry.1 else {
        todo!()
    };
    todo!("substitute implicit symbols")
}

pub(crate) fn ty_path_ty_method_cards(
    db: &dyn TermDb,
    path: TypePath,
) -> EntityTreeBundleResultRef<&[(Ident, Result<TypeMethodCard, ()>)]> {
    match ty_path_ty_method_cards_aux(db, path) {
        Ok(ty_method_cards) => Ok(ty_method_cards),
        Err(e) => Err(e),
    }
}

#[salsa::tracked(jar = TermJar, return_ref)]
pub(crate) fn ty_path_ty_method_cards_aux(
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
    #[return_ref]
    pub method_ty_info_inner: TermResult<MethodTypeInfo>,
    pub method_ty: TermResult<Term>,
}

impl TypeMethodCard {
    fn new(db: &dyn TermDb, decl: TypeMethodDecl) -> Self {
        let id = decl.associated_item(db).id(db);
        let signature = ty_method_signature(db, decl);
        let method_ty_info: TermResult<MethodTypeInfo> =
            MethodTypeInfo::new_ty_method_ty_info(db, signature);
        let method_ty = method_ty_info
            .as_ref()
            .map_err(|e| *e)
            .map(|ty_info| ty_info.ty(db))
            .flatten();
        Self::new_inner(db, id, decl, signature, method_ty_info, method_ty)
    }

    pub fn method_ty_info<'a>(self, db: &'a dyn TermDb) -> TermResult<&'a MethodTypeInfo> {
        match self.method_ty_info_inner(db) {
            Ok(ty_info) => Ok(ty_info),
            Err(e) => Err(*e),
        }
    }
}

impl MethodTypeInfo {
    fn new_ty_method_ty_info(
        db: &dyn TermDb,
        signature: SignatureResult<TypeMethodSignature>,
    ) -> TermResult<Self> {
        // todo: formal method, method that is not a function pointer
        let signature = signature?;
        let t = |parameter_sig:& RegularParameterSignature| -> TermResult<TermRitchieParameterLiasonedType>{
            Ok(TermRitchieParameterLiasonedType::new(
                Term::from_raw_unchecked(
                    db,
                    parameter_sig.ty(),
                    TermTypeExpectation::FinalDestinationEqsSort,
                )?,
            ))
        };
        let self_liasoned_ty = t(signature.self_parameter(db))?;
        let nonself_parameter_liasoned_tys = signature
            .nonself_regular_parameters(db)
            .iter()
            .map(t)
            .collect::<TermResult<Vec<_>>>()?;
        let return_ty = Term::from_raw(
            db,
            signature.return_ty(db),
            TermTypeExpectation::FinalDestinationEqsSort,
        )?;
        let implicit_parameters = signature
            .implicit_parameters(db)
            .iter()
            .map(|_| todo!())
            .collect();
        Ok(Self {
            implicit_parameters,
            self_liasoned_ty,
            nonself_parameter_liasoned_tys,
            return_ty,
            where_clause: (),
        })
    }
}
