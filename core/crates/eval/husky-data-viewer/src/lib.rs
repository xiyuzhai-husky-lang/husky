mod query;

use husky_word::IdentPairDict;
pub use query::*;

use husky_vm_interface::{__LinkageFp, __Register};

#[derive(Debug, PartialEq, Eq)]
pub enum HuskyDataViewer {
    Struct {
        fields: IdentPairDict<__LinkageFp>,
    },
    Vec {
        len: __LinkageFp,
        index: __LinkageFp,
    },
}

impl HuskyDataViewer {
    pub fn print<'eval>(&self, value: &__Register<'eval>) -> String {
        todo!()
    }

    pub fn serialize<'eval>(&self, value: &__Register<'eval>) -> String {
        todo!()
    }

    pub fn member_iter<'eval>(
        &self,
        value: &__Register<'eval>,
    ) -> impl Iterator<Item = __Register<'eval>> {
        (0..250).into_iter().map(|_| todo!())
    }
}
