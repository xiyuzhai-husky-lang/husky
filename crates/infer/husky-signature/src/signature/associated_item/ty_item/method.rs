use husky_entity_tree::{ImplBlock, ImplBlockId};

use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub fn ty_method_signature(
    db: &dyn SignatureDb,
    decl: TypeMethodFnDecl,
) -> SignatureResult<TypeMethodSignature> {
    let impl_block = decl.associated_item(db).impl_block(db);
    let explicit_parameter_list = decl.explicit_parameter_decl_list(db);
    let self_parameter = match impl_block {
        ImplBlock::Type(impl_block) => {
            ExplicitParameterSignature::new(todo!(), impl_block.signature(db)?.ty(db))
        }
        _ => unreachable!(),
    };
    let expr_region = decl.expr_region(db);
    let signature_term_region = signature_term_region(db, expr_region);
    let raw_term_menu = db.raw_term_menu(expr_region.toolchain(db)).unwrap();
    let implicit_parameters = ImplicitParameterSignatures::from_decl(
        decl.implicit_parameters(db),
        signature_term_region,
        raw_term_menu,
    );
    let nonself_regular_parameters =
        ExplicitParameterSignatures::from_decl(decl.regular_parameters(db), signature_term_region)?;
    let return_ty = match decl.return_ty(db) {
        Some(return_ty) => signature_term_region.expr_term(return_ty.expr())?,
        None => raw_term_menu.unit(),
    };
    Ok(TypeMethodSignature::new(
        db,
        implicit_parameters,
        self_parameter,
        nonself_regular_parameters,
        return_ty,
    ))
}

#[salsa::interned(db = SignatureDb, jar = SignatureJar)]
pub struct TypeMethodSignature {
    // todo: formal method, method that is not a function pointer
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatures,
    #[return_ref]
    pub self_parameter: ExplicitParameterSignature,
    #[return_ref]
    pub nonself_regular_parameters: ExplicitParameterSignatures,
    pub return_ty: RawTerm,
}
