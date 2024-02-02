use super::*;

#[salsa::tracked(db = DecTermDb, jar = DecTermJar)]
pub struct ListDecTerm {
    pub toolchain: Toolchain,
    #[return_ref]
    pub items: Vec<DecTerm>,
}

impl ListDecTerm {
    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
        ctx: &mut DecTermShowContext,
    ) -> std::fmt::Result {
        f.write_str("[")?;
        let items = self.items(db);
        for (i, item) in items.iter().enumerate() {
            item.show_with_db_fmt(f, db, ctx)?;
            if i < items.len() {
                f.write_str(", ")?
            }
        }
        f.write_str("]")
    }
}
