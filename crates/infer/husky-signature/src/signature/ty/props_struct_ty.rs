use super::*;

#[salsa::tracked(jar = SignatureJar)]
pub struct PropsStructTypeSignature {
    #[return_ref]
    pub implicit_parameter_decl_list: Option<ImplicitParameterSignatureList>,
    #[return_ref]
    pub fields: Vec<PropsStructFieldSignature>,
}

impl PropsStructTypeSignature {
    pub fn implicit_parameters(self, db: &dyn SignatureDb) -> &[ImplicitParameterSignature] {
        self.implicit_parameter_decl_list(db)
            .as_ref()
            .map(|l| -> &[ImplicitParameterSignature] { &l })
            .unwrap_or(&[])
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct PropsStructFieldSignature {
    ident: Identifier,
    ty: Term,
}
