use crate::{version_stamp::ValVersionStamp, Val};

#[salsa::jar(db = ValDb)]
pub struct ValJar(Val, ValVersionStamp);
