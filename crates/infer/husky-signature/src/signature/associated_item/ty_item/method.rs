use husky_entity_tree::{ImplBlock, ImplBlockId};

use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub fn ty_method_signature(
    db: &dyn SignatureDb,
    decl: TypeMethodFnDecl,
) -> SignatureResult<TypeMethodSignature> {
    let impl_block = decl.associated_item(db).impl_block(db);
    let self_parameter = match impl_block {
        ImplBlock::Type(impl_block) => {
            let ty_impl_block_signature =
                ty_impl_block_signature(db, ty_impl_block_decl(db, impl_block)?)?;
            RegularParameterSignature::new_self_parameter(ty_impl_block_signature.ty(db))
        }
        ImplBlock::TraitForType(_) | ImplBlock::IllFormed(_) => unreachable!(),
    };
    let expr_region = decl.expr_region(db);
    let signature_term_region = signature_term_region(db, expr_region);
    let raw_term_menu = db.raw_term_menu(expr_region.toolchain(db)).unwrap();
    let implicit_parameters = ImplicitParameterSignatures::from_decl(
        decl.implicit_parameters(db)?,
        signature_term_region,
        raw_term_menu,
    );
    let parameters =
        RegularParameterSignatures::from_decl(decl.regular_parameters(db)?, signature_term_region)?;
    let return_ty = match decl.return_ty(db) {
        Ok(return_ty) => match signature_term_region.expr_term(return_ty.expr()) {
            Ok(return_ty) => return_ty,
            Err(_) => todo!(),
        },
        Err(_) => todo!(), //  Err(SignatureRawTermAbortion::ExprError),
    };
    Ok(TypeMethodSignature::new(
        db,
        implicit_parameters,
        self_parameter,
        parameters,
        return_ty,
    ))
}

#[salsa::interned(db = SignatureDb, jar = SignatureJar)]
pub struct TypeMethodSignature {
    // todo: formal method, method that is not a function pointer
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatures,
    #[return_ref]
    pub self_parameter: RegularParameterSignature,
    #[return_ref]
    pub nonself_regular_parameters: RegularParameterSignatures,
    pub return_ty: RawTerm,
}
