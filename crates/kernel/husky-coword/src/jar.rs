use crate::*;

#[salsa::jar]
pub struct CowordJar(
    Coword,
    coword_menu,
    kebab_to_ident,
    is_coword_valid_kebab,
    ident_to_name,
);
