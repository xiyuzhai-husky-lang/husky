use entity_route::RangedEntityRoute;
use word::RangedCustomIdentifier;

use crate::*;

use super::*;

// inner ops
impl<'a> AtomLRParser<'a> {
    pub(crate) fn lambda_head(
        &mut self,
    ) -> AtomResult<Vec<(RangedCustomIdentifier, Option<RangedEntityRoute>)>> {
        Ok(comma_list!(self, lambda_parameter!, "|"))
    }

    pub(crate) fn lambda_parameter(
        &mut self,
    ) -> AtomResult<(RangedCustomIdentifier, Option<RangedEntityRoute>)> {
        let ident = get!(self, custom_ident);
        let ty = if next_matches!(self, ":") {
            Some(RangedEntityRoute {
                route: get!(self.ty?),
                range: self.stream.pop_range(),
            })
        } else {
            None
        };
        Ok((ident, ty))
    }
}
