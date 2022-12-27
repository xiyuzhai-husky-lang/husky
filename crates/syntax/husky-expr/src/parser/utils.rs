use salsa::DebugWithDb;

use super::*;

impl<'a, 'b, 'c> ExprParser<'a, 'b, 'c> {
    pub(super) fn report_position(&self) -> String {
        format!("{:?}", self.ctx.entity_path().debug(self.ctx.db()))
    }
}
