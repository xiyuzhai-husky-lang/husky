use super::*;
use expr::VdSynExprArenaRef;

impl<'a, 'db> VdSynExprParser<'a, 'db> {
    pub fn show(&self) -> String {
        let db = self.db();
        format!(
            "{}",
            self.builder
                .with_expr_arena(|expr_arena| self.stack.show(expr_arena.as_arena_ref(), db))
        )
    }
}
