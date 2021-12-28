use crate::*;

use super::*;

// inner ops
impl<'a> ScopeLRParser<'a> {
    pub(crate) fn lambda_head(&mut self) -> AtomResult<Vec<(CustomIdentifier, Option<ScopeId>)>> {
        Ok(comma_list!(self, lambda_parameter!, "|"))
    }

    pub(crate) fn lambda_parameter(&mut self) -> AtomResult<(CustomIdentifier, Option<ScopeId>)> {
        let ident = get!(self, custom_ident);
        let ty = if next_matches!(self, ":") {
            Some(get!(self.ty?))
        } else {
            None
        };
        Ok((ident, ty))
    }
}
