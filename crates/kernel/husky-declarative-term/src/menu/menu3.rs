use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct DeclarativeTermMenu3 {
    parent: DeclarativeTermMenu2,
}

impl std::ops::Deref for DeclarativeTermMenu3 {
    type Target = DeclarativeTermMenu2;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DeclarativeTermMenu3 {
    pub(crate) fn new(
        db: &dyn DeclarativeTermDb,
        toolchain: Toolchain,
        _menu2: DeclarativeTermMenu2,
    ) -> DeclarativeTermResult<Self> {
        let menu0 = DeclarativeTermMenu0::new(db, toolchain);
        let menu1 = DeclarativeTermMenu1::new(db, toolchain, menu0);
        let menu2 = DeclarativeTermMenu2::new(db, toolchain, menu1)?;
        Ok(DeclarativeTermMenu3 { parent: menu2 })
    }
}

type A = core::primitive::i32;
