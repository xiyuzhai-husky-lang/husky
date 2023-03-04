use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct ValidTermMenu3 {
    parent: ValidTermMenu2,
}

impl std::ops::Deref for ValidTermMenu3 {
    type Target = ValidTermMenu2;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl ValidTermMenu3 {
    pub(crate) fn new(
        db: &dyn ValidTermDb,
        toolchain: Toolchain,
        _menu2: ValidTermMenu2,
    ) -> ValidTermResult<Self> {
        let menu0 = ValidTermMenu0::new(db, toolchain);
        let menu1 = ValidTermMenu1::new(db, toolchain, menu0);
        let menu2 = ValidTermMenu2::new(db, toolchain, menu1)?;
        Ok(ValidTermMenu3 { parent: menu2 })
    }
}

type A = core::primitive::i32;
