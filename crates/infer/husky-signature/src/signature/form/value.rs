use crate::*;

#[salsa::tracked(jar = SignatureJar,return_ref)]
pub fn value_signature(db: &dyn SignatureDb, decl: ValueDecl) -> SignatureResult<ValueSignature> {
    let expr_region = decl.expr_region(db);
    let signature_term_region = signature_term_region(db, expr_region);
    let raw_term_menu = db
        .raw_term_menu(expr_region.toolchain(db))
        .as_ref()
        .unwrap();
    Ok(ValueSignature::new(db))
}

#[salsa::interned(db = SignatureDb, jar = SignatureJar)]
pub struct ValueSignature {}

impl ValueSignature {
    pub fn implicit_parameters(self, db: &dyn SignatureDb) -> &[ImplicitParameterSignature] {
        todo!()
        // self.implicit_parameter_decl_list(db)
        //     .as_ref()
        //     .map(|l| -> &[ImplicitParameterSignature] { &l })
        //     .unwrap_or(&[])
    }
}
