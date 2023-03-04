use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct PreciseTermMenu3 {
    parent: PreciseTermMenu2,
}

impl std::ops::Deref for PreciseTermMenu3 {
    type Target = PreciseTermMenu2;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl PreciseTermMenu3 {
    pub(crate) fn new(
        db: &dyn PreciseTermDb,
        toolchain: Toolchain,
        _menu2: PreciseTermMenu2,
    ) -> PreciseTermResult<Self> {
        let menu0 = PreciseTermMenu0::new(db, toolchain);
        let menu1 = PreciseTermMenu1::new(db, toolchain, menu0);
        let menu2 = PreciseTermMenu2::new(db, toolchain, menu1)?;
        Ok(PreciseTermMenu3 { parent: menu2 })
    }
}

type A = core::primitive::i32;
