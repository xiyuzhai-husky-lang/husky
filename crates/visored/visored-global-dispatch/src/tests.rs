use crate::*;

#[salsa::db(
    husky_coword::jar::CowordJar,
    latex_vfs::jar::LxVfsJar,
    visored_entity_path::jar::VdEntityPathJar,
    visored_opr::jar::VdOprJar,
    visored_term::jar::VdTermJar,
    visored_signature::jar::VdSignatureJar,
    Jar
)]
pub(crate) struct DB();
