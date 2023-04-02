mod form_body;
mod module_items;
mod trai_for_ty_items;
mod trai_items;
mod ty_items;
mod variants;

pub use self::form_body::*;
pub use self::module_items::*;
pub use self::trai_for_ty_items::*;
pub use self::trai_items::*;
pub use self::ty_items::*;
pub use self::variants::*;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum DefnBody {
    FormBody(FormBody),
    Variants(Variants)
}

impl DefnBody {
    pub fn ast_idx_range(self) -> AstIdxRange {
        todo!()
    }

    pub fn form_body(self) -> Option<FormBody> {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ImplBlockItems {}

impl ImplBlockItems {
    pub fn ast_idx_range(self) -> AstIdxRange {
        todo!()
    }
}
