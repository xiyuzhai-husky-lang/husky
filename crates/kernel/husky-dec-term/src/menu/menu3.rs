use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct DecTermMenu3 {
    parent: DecTermMenu2,
}

impl std::ops::Deref for DecTermMenu3 {
    type Target = DecTermMenu2;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DecTermMenu3 {
    pub(crate) fn new(
        db: &::salsa::Db,
        toolchain: Toolchain,
        _menu2: DecTermMenu2,
    ) -> DecTermResult<Self> {
        let menu0 = DecTermMenu0::new(db, toolchain);
        let menu1 = DecTermMenu1::new(db, toolchain, menu0);
        let menu2 = DecTermMenu2::new(db, toolchain, menu1)?;
        Ok(DecTermMenu3 { parent: menu2 })
    }
}
