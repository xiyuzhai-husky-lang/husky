use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
// #[salsa::debug_with_db(db = EntitySynTreeDb)]
pub struct SelfParameterObelisk {
    ephem_symbol_modifier_token_group: Option<RegionalEphemSymbolModifierTokenGroup>,
    self_value_token: RegionalSelfValueToken,
}

impl SelfParameterObelisk {
    pub fn ephem_symbol_modifier_token_group(
        &self,
    ) -> Option<RegionalEphemSymbolModifierTokenGroup> {
        self.ephem_symbol_modifier_token_group
    }

    pub fn self_value_token(&self) -> RegionalSelfValueToken {
        self.self_value_token
    }
}

impl<'a, 'b> TryParseOptionFromStream<SynDeclExprParser<'a>> for SelfParameterObelisk {
    type Error = SynExprError;

    // needs more testing
    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut SynDeclExprParser<'a>,
    ) -> Result<Option<Self>, Self::Error> {
        let ephem_symbol_modifier_token_group = ctx.try_parse_option()?;
        if let Some(self_value_token) = ctx.try_parse_option::<RegionalSelfValueToken>()? {
            Ok(Some(Self {
                ephem_symbol_modifier_token_group,
                self_value_token,
            }))
        } else {
            Ok(None)
        }
    }
}
