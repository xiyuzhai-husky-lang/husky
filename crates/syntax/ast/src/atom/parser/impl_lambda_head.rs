use entity_route::RangedScope;

use crate::*;

use super::*;

// inner ops
impl<'a> AtomLRParser<'a> {
    pub(crate) fn lambda_head(
        &mut self,
    ) -> AstResult<Vec<(CustomIdentifier, Option<RangedScope>)>> {
        Ok(comma_list!(self, lambda_parameter!, "|"))
    }

    pub(crate) fn lambda_parameter(
        &mut self,
    ) -> AstResult<(CustomIdentifier, Option<RangedScope>)> {
        let ident = get!(self, custom_ident);
        let ty = if next_matches!(self, ":") {
            Some(RangedScope {
                scope: get!(self.ty?),
                range: self.stream.pop_range(),
            })
        } else {
            None
        };
        Ok((ident, ty))
    }
}
