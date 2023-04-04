use crate::*;

#[salsa::interned(db = SignatureDb, jar = SignatureJar)]
pub struct ValueSignature {}

impl ValueSignature {
    pub fn implicit_parameters(self, _db: &dyn SignatureDb) -> &[ImplicitParameterSignature] {
        todo!()
        // self.implicit_parameter_decl_list(db)
        //     .as_ref()
        //     .map(|l| -> &[ImplicitParameterSignature] { &l })
        //     .unwrap_or(&[])
    }
}

impl HasSignature for ValueDecl {
    type Signature = ValueSignature;

    fn signature(self, db: &dyn SignatureDb) -> SignatureResult<Self::Signature> {
        value_signature(db, self)
    }
}

#[salsa::tracked(jar = SignatureJar)]
pub fn value_signature(db: &dyn SignatureDb, decl: ValueDecl) -> SignatureResult<ValueSignature> {
    let expr_region = decl.expr_region(db);
    let _signature_term_region = signature_term_region(db, expr_region);
    let _raw_term_menu = db.raw_term_menu(expr_region.toolchain(db)).unwrap();
    Ok(ValueSignature::new(db))
}
