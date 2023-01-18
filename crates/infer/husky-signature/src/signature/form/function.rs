use husky_token::{CurryToken, EolColonToken};

use crate::*;

#[salsa::interned(jar = SignatureJar)]
pub struct FunctionSignature {
    #[return_ref]
    pub implicit_parameter_decl_list: Option<ImplicitParameterSignatureList>,
    #[return_ref]
    pub parameter_decl_list: ParameterSignatureList,
    #[return_ref]
    pub curry_token: SignatureResult<CurryToken>,
    #[return_ref]
    pub output_ty: SignatureResult<ExprIdx>,
    #[return_ref]
    pub eol_colon: SignatureResult<EolColonToken>,
}

impl FunctionSignature {
    pub fn implicit_parameters(self, db: &dyn SignatureDb) -> &[ImplicitParameterSignature] {
        self.implicit_parameter_decl_list(db)
            .as_ref()
            .map(|l| -> &[ImplicitParameterSignature] { &l })
            .unwrap_or(&[])
    }
}
