use super::*;
use std::fmt::{Debug, Display};

#[salsa::interned(db = RawTermDb, jar = RawTermJar)]
pub struct RawTermExplicitApplicationOrRitchieCall {
    pub function: RawTerm,
    #[return_ref]
    pub implicit_arguments: Vec<RawTerm>,
    #[return_ref]
    pub items: Vec<RawTerm>,
}

impl RawTermExplicitApplicationOrRitchieCall {
    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn RawTermDb,
        ctx: &mut RawTermShowContext,
    ) -> std::fmt::Result {
        todo!()
    }
}
