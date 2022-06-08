use entity_route::RangedEntityRoute;
use text::RangedCustomIdentifier;

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
        let ident = get!(self, custom_ident);
        let ty = if try_eat!(self, ":") {
            Some(get!(self.ranged_ty?))
        } else {
            None
        };
        Ok((ident, ty))
    }
}
