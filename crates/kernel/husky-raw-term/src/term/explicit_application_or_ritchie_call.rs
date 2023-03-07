use super::*;
use std::fmt::{Debug, Display};

#[salsa::interned(db = RawTermDb, jar = RawTermJar)]
pub struct RawTermExplicitApplicationOrRitchieCall {
    pub function: RawTerm,
    #[return_ref]
    pub implicit_arguments: Vec<RawTerm>,
    #[return_ref]
    pub items: Vec<RawTerm>,
    pub extra_comma: bool,
}

impl RawTermExplicitApplicationOrRitchieCall {
    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn RawTermDb,
        ctx: &mut RawTermShowContext,
    ) -> std::fmt::Result {
        self.function(db).show_with_db_fmt(f, db, ctx);
        let implicit_arguments = self.implicit_arguments(db);
        if implicit_arguments.len() > 0 {
            todo!()
        }
        f.write_str("(");
        let items = self.items(db);
        let extra_comma = self.extra_comma(db);
        for (i, parameter_ty) in items.iter().enumerate() {
            parameter_ty.show_with_db_fmt(f, db, ctx)?;
            if i < items.len() {
                f.write_str(", ")?
            } else if extra_comma {
                f.write_str(",")?
            }
        }
        Ok(())
    }
}
