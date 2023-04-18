use husky_entity_tree::ImplBlock;

use crate::*;

#[salsa::tracked(jar = DeclarativeSignatureJar)]
pub(crate) fn trai_for_ty_method_signature(
    db: &dyn DeclarativeSignatureDb,
    decl: TraitForTypeMethodDecl,
) -> DeclarativeSignatureResult<TraitForTypeMethodSignature> {
    let self_parameter = {
        let impl_block = decl.associated_item(db).impl_block(db);
        let contract = match decl.self_parameter(db) {
            Some(self_parameter) => todo!(),
            None => Contract::Pure,
        };
        match impl_block {
            ImplBlock::TraitForType(impl_block) => ExplicitParameterSignature::new(
                contract,
                impl_block.declarative_signature(db)?.ty(db),
            ),
            ImplBlock::Type(_) | ImplBlock::IllFormed(_) => unreachable!(),
        }
    };
    let expr_region = decl.expr_region(db);
    let expr_region_data = expr_region.data(db);
    let declarative_term_region = declarative_term_region(db, expr_region);
    let declarative_term_menu = db.declarative_term_menu(expr_region.toolchain(db)).unwrap();
    let implicit_parameters = ImplicitParameterSignatures::from_decl(
        decl.implicit_parameters(db),
        declarative_term_region,
        declarative_term_menu,
    );
    let nonself_regular_parameters = ExplicitParameterSignatures::from_decl(
        decl.regular_parameters(db),
        expr_region_data,
        declarative_term_region,
    )?;
    let return_ty = match decl.return_ty(db) {
        Some(return_ty) => declarative_term_region.expr_term(return_ty.expr())?,
        None => declarative_term_menu.unit(),
    };
    Ok(TraitForTypeMethodSignature::new(
        db,
        implicit_parameters,
        self_parameter,
        nonself_regular_parameters,
        return_ty,
    ))
}

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct TraitForTypeMethodSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatures,
    #[return_ref]
    pub self_parameter: ExplicitParameterSignature,
    #[return_ref]
    pub nonself_regular_parameters: ExplicitParameterSignatures,
    pub return_ty: DeclarativeTerm,
}
