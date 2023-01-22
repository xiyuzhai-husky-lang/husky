use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub(crate) fn ty_associated_function_signature(
    db: &dyn SignatureDb,
    decl: TypeAssociatedFunctionDecl,
) -> TypeAssociatedFunctionSignature {
    let expr_region = decl.expr_region(db);
    let signature_term_region = signature_term_region(db, expr_region);
    let term_menu = db.term_menu(expr_region.toolchain(db)).as_ref().unwrap();
    TypeAssociatedFunctionSignature::new(db, todo!())
}

#[salsa::tracked(jar = SignatureJar)]
pub struct TypeAssociatedFunctionSignature {
    pub output_ty: Term,
}
