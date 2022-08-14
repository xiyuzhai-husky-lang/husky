use husky_entity_route::RangedEntityRoute;
use husky_text::RangedCustomIdentifier;

use crate::*;

use super::*;

// inner ops
impl<'a, 'b> AtomParser<'a, 'b> {
    pub(crate) fn lambda_head(
        &mut self,
    ) -> AtomResult<Vec<(RangedCustomIdentifier, Option<RangedEntityRoute>)>> {
        Ok(comma_list!(self, lambda_parameter!, "|"))
    }

    pub(crate) fn lambda_parameter(
        &mut self,
    ) -> AtomResult<(RangedCustomIdentifier, Option<RangedEntityRoute>)> {
        let ident = deprecated_get!(self, custom_ident);
        let ty = if deprecated_try_eat!(self, ":") {
            Some(deprecated_get!(self.ranged_ty?))
        } else {
            None
        };
        Ok((ident, ty))
    }
}
