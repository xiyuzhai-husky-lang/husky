use super::*;
use std::fmt::Debug;

#[salsa::interned(db = DecTermDb, jar = DecTermJar, constructor = new_inner)]
pub struct DecApplicationOrRitchieCall {
    pub function: DecTerm,
    #[return_ref]
    pub generic_arguments: Vec<DecTerm>,
    #[return_ref]
    pub items: Vec<DecTerm>,
    pub extra_comma: bool,
}

impl DecApplicationOrRitchieCall {
    pub fn new(
        function: DecTerm,
        template_arguments: Vec<DecTerm>,
        items: Vec<DecTerm>,
        extra_comma: bool,
        db: &::salsa::Db,
    ) -> Self {
        Self::new_inner(db, function, template_arguments, items, extra_comma)
    }

    #[inline(never)]
    pub(crate) fn display_fmt_with_db_and_ctx(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
        ctx: &DecSymbolNameMap,
    ) -> std::fmt::Result {
        self.function(db).display_fmt_with_db_and_ctx(f, db, ctx)?;
        let generic_arguments = self.generic_arguments(db);
        if generic_arguments.len() > 0 {
            todo!()
        }
        f.write_str("(")?;
        let items = self.items(db);
        let extra_comma = self.extra_comma(db);
        for (i, parameter_ty) in items.iter().enumerate() {
            parameter_ty.display_fmt_with_db_and_ctx(f, db, ctx)?;
            if i < items.len() {
                f.write_str(", ")?
            } else if extra_comma {
                f.write_str(",")?
            }
        }
        Ok(())
    }
}
