use super::*;

#[salsa::tracked(db = RawTermDb, jar = RawTermJar)]
pub struct RawTermList {
    pub toolchain: Toolchain,
    #[return_ref]
    pub items: Vec<RawTerm>,
}

impl RawTermList {
    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn RawTermDb,
        ctx: &mut RawTermShowContext,
    ) -> std::fmt::Result {
        todo!()
    }
}
