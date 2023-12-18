use super::*;
use std::fmt::Debug;

#[salsa::interned(db = DeclarativeTermDb, jar = DeclarativeTermJar, constructor = new_inner)]
pub struct DeclarativeTermExplicitApplicationOrRitchieCall {
    pub function: DeclarativeTerm,
    #[return_ref]
    pub generic_arguments: Vec<DeclarativeTerm>,
    #[return_ref]
    pub items: Vec<DeclarativeTerm>,
    pub extra_comma: bool,
}

impl DeclarativeTermExplicitApplicationOrRitchieCall {
    pub fn new(
        function: DeclarativeTerm,
        template_arguments: Vec<DeclarativeTerm>,
        items: Vec<DeclarativeTerm>,
        extra_comma: bool,
        db: &::salsa::Db,
    ) -> Self {
        use husky_print_utils::p;
        use salsa::DebugWithDb;
        p!(
            function.debug(db),
            template_arguments.debug(db),
            items.debug(db)
        );
        assert!(!items.is_empty());
        Self::new_inner(db, function, template_arguments, items, extra_comma)
    }

    #[inline(never)]
    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
        ctx: &mut DeclarativeTermShowContext,
    ) -> std::fmt::Result {
        self.function(db).show_with_db_fmt(f, db, ctx);
        let generic_arguments = self.generic_arguments(db);
        if generic_arguments.len() > 0 {
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
