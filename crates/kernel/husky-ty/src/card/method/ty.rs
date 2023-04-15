use husky_decl::HasItemDecls;
use husky_entity_tree::EntityTreeBundleResultRef;
use husky_signature::{ty_method_signature, ExplicitParameterSignature};
use vec_like::VecMapGetEntry;

use super::*;

#[salsa::tracked(db = TypeDb, jar = TypeJar, constructor = new_inner)]
pub struct TypeMethodFnCard {
    #[id]
    pub id: AssociatedItemId,
    #[return_ref]
    pub method_ty_info_inner: TermResult<MethodTypeInfo>,
    pub method_ty: TermResult<Term>,
}

pub trait HasTypeMethodCard: Copy {
    fn ty_method_card(self, db: &dyn TypeDb, ident: Ident) -> TermResult<Option<TypeMethodFnCard>>;
}

impl HasTypeMethodCard for Term {
    fn ty_method_card(self, db: &dyn TypeDb, ident: Ident) -> TermResult<Option<TypeMethodFnCard>> {
        // using the fact that owner_ty is reduced
        match self {
            Term::EntityPath(TermEntityPath::TypeOntology(path)) => {
                ty_ontology_path_ty_method_card(db, path, ident)
            }
            Term::Application(raw_ty) => term_application_ty_method_card(db, raw_ty, ident),
            _ => Ok(None),
        }
    }
}

impl TypeMethodFnCard {
    fn new(db: &dyn TypeDb, decl: TypeMethodFnDecl) -> Self {
        todo!()
        // let id = decl.associated_item(db).id(db);
        // let signature = ty_method_signature(db, decl);
        // let method_ty_info: TermResult<MethodTypeInfo> =
        //     MethodTypeInfo::new_ty_method_ty_info(db, signature);
        // let method_ty = method_ty_info
        //     .as_ref()
        //     .map_err(|e| *e)
        //     .map(|ty_info| ty_info.ty(db))
        //     .flatten();
        // Self::new_inner(db, id, method_ty_info, method_ty)
    }

    pub fn method_ty_info<'a>(self, db: &'a dyn TypeDb) -> TermResult<&'a MethodTypeInfo> {
        match self.method_ty_info_inner(db) {
            Ok(ty_info) => Ok(ty_info),
            Err(e) => Err(*e),
        }
    }
}

impl MethodTypeInfo {
    fn new_ty_method_ty_info(
        db: &dyn TypeDb,
        signature: SignatureResult<TypeMethodSignature>,
    ) -> TermResult<Self> {
        // todo: formal method, method that is not a function pointer
        let signature = signature?;
        let t =
            |param: &ExplicitParameterSignature| -> TermResult<TermRitchieParameterContractedType> {
                Ok(TermRitchieParameterContractedType::new(
                    param.contract(),
                    Term::from_raw(db, param.ty(), TermTypeExpectation::FinalDestinationEqsSort)?,
                ))
            };
        let self_contracted_ty = t(signature.self_parameter(db))?;
        let nonself_parameter_contracted_tys = signature
            .nonself_regular_parameters(db)
            .iter()
            .map(t)
            .collect::<TermResult<Vec<_>>>()?;
        let return_ty = Term::ty_from_raw(db, signature.return_ty(db))?;
        let implicit_parameters = signature
            .implicit_parameters(db)
            .iter()
            .map(|_| todo!())
            .collect();
        Ok(Self {
            implicit_parameters,
            self_contracted_ty,
            nonself_parameter_contracted_tys,
            return_ty,
            where_clause: (),
        })
    }
}

pub(crate) fn ty_ontology_path_ty_method_card(
    db: &dyn TypeDb,
    path: TypePath,
    ident: Ident,
) -> TermResult<Option<TypeMethodFnCard>> {
    let ty_method_cards = ty_path_ty_method_cards(db, path)?;
    let Some(entry) = ty_method_cards.get_entry(ident) else {
        return Ok(None)
    };
    let Ok(ty_method_card) = entry.1 else {
        todo!()
    };
    Ok(Some(ty_method_card))
}

#[salsa::tracked(jar = TypeJar)]
pub(crate) fn term_application_ty_method_card(
    db: &dyn TypeDb,
    raw_ty: TermApplication,
    ident: Ident,
) -> TermResult<Option<TypeMethodFnCard>> {
    todo!()
    // let application_expansion = application_expansion_salsa(db, raw_ty);
    // let function = application_expansion.function();
    // match function {
    //     TermFunctionReduced::TypeOntology(path) => ty_ontology_path_application_ty_method_card(
    //         db,
    //         path,
    //         application_expansion.opt_arguments(db).unwrap(),
    //         ident,
    //     ),
    //     TermFunctionReduced::Trait(_) => todo!(),
    //     TermFunctionReduced::Other(_) => todo!(),
    // }
}

fn ty_ontology_path_application_ty_method_card(
    db: &dyn TypeDb,
    path: TypePath,
    _arguments: &[Term],
    ident: Ident,
) -> TermResult<Option<TypeMethodFnCard>> {
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
    db: &dyn TypeDb,
    path: TypePath,
) -> EntityTreeBundleResultRef<&[(Ident, Result<TypeMethodFnCard, ()>)]> {
    match ty_path_ty_method_cards_aux(db, path) {
        Ok(ty_method_cards) => Ok(ty_method_cards),
        Err(e) => Err(e),
    }
}

#[salsa::tracked(jar = TypeJar, return_ref)]
pub(crate) fn ty_path_ty_method_cards_aux(
    db: &dyn TypeDb,
    path: TypePath,
) -> EntityTreeBundleResult<IdentPairMap<Result<TypeMethodFnCard, ()>>> {
    Ok(path
        .item_decls(db)?
        .iter()
        .copied()
        .filter_map(|(ident, decl)| {
            Some((
                ident,
                match decl {
                    Ok(TypeItemDecl::MethodFn(decl)) => Ok(TypeMethodFnCard::new(db, decl)),
                    Ok(_) => return None,
                    Err(_) => Err(()),
                },
            ))
        })
        .collect())
}
