mod menu0;
mod menu1;
mod menu2;
mod menu3;
mod menu4;
mod menu5;

pub use menu0::*;
pub use menu1::*;
pub use menu2::*;
pub use menu3::*;
pub use menu4::*;
pub use menu5::*;

use crate::*;
use std::sync::Arc;

#[derive(Debug, PartialEq, Eq)]
pub struct TermMenu {
    parent: TermMenu5,
}

impl std::ops::Deref for TermMenu {
    type Target = TermMenu5;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl TermMenu {
    pub(crate) fn new(db: &dyn TermDb) -> Arc<TermMenu> {
        let menu0 = TermMenu0::new(db);
        todo!();
        let menu1 = TermMenu1::new(db, menu0);
        let menu2 = TermMenu2::new(db, menu1);
        let menu3 = TermMenu3::new(db, menu2);
        let menu4 = TermMenu4::new(db, menu3);
        let menu5 = TermMenu5::new(db, menu4);
        Arc::new(TermMenu { parent: menu5 })
    }
}
