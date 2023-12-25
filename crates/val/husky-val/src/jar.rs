use crate::{version_stamp::ValVersionStamp, *};

#[salsa::jar(db = ValDb)]
pub struct ValJar(
    Val,
    ValRuntimeConstant,
    ValRuntimeConstants,
    ValVersionStamp,
);
