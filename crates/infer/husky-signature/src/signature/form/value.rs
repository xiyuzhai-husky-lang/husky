use crate::*;

#[salsa::interned(jar = SignatureJar)]
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
