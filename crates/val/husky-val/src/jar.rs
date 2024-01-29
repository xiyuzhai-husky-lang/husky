use crate::{version_stamp::ValVersionStamp, *};

#[salsa::jar]
pub struct ValJar(Val, ValPattern, ValRuntimeConstant, ValVersionStamp);
