use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct RawTermMenu3 {
    parent: RawTermMenu2,
}

impl std::ops::Deref for RawTermMenu3 {
    type Target = RawTermMenu2;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl RawTermMenu3 {
    pub(crate) fn new(
        db: &dyn RawTermDb,
        toolchain: Toolchain,
        _menu2: RawTermMenu2,
    ) -> RawTermResult<Self> {
        let menu0 = RawTermMenu0::new(db, toolchain);
        let menu1 = RawTermMenu1::new(db, toolchain, menu0);
        let menu2 = RawTermMenu2::new(db, toolchain, menu1)?;
        Ok(RawTermMenu3 { parent: menu2 })
    }
}

type A = core::primitive::i32;
