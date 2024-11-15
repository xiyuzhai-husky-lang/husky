#[salsa::jar]
pub struct VdSignatureJar(
    crate::signature::separator::base::VdBaseSeparatorSignature,
    crate::menu::vd_signature_menu,
);
