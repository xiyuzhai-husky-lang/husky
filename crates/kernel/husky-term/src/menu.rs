mod menu0;
mod menu1;
mod menu2;
mod menu3;

pub use menu0::*;
pub use menu1::*;
pub use menu2::*;
pub use menu3::*;

use crate::*;
use std::sync::Arc;

#[derive(Debug, PartialEq, Eq)]
pub struct TermMenu {
    parent: TermMenu3,
}
impl std::ops::Deref for TermMenu {
    type Target = TermMenu3;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}
pub(crate) fn term_menu(db: &dyn TermDb) -> Arc<TermMenu> {
    let menu0 = TermMenu0::new(db);
    let menu1 = TermMenu1::new(db, menu0);
    let menu2 = TermMenu2::new(db, menu1);
    let menu3 = TermMenu3::new(db, menu2);
    Arc::new(TermMenu { parent: menu3 })
}
