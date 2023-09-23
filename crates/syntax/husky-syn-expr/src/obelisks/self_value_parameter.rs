use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
// #[salsa::debug_with_db(db = EntitySynTreeDb)]
pub struct SelfParameterObelisk {
    ephem_symbol_modifier_token_group: Option<EphemSymbolModifierRegionalTokenGroup>,
    self_value_token: SelfValueRegionalToken,
}

impl SelfParameterObelisk {
    pub fn ephem_symbol_modifier_token_group(
        &self,
    ) -> Option<EphemSymbolModifierRegionalTokenGroup> {
        self.ephem_symbol_modifier_token_group
    }

    pub fn self_value_token(&self) -> SelfValueRegionalToken {
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
        if let Some(ephem_symbol_modifier_token_group) = ephem_symbol_modifier_token_group {
            match ephem_symbol_modifier_token_group {
                EphemSymbolModifierRegionalTokenGroup::RefMut(_, None, _)
                | EphemSymbolModifierRegionalTokenGroup::Ambersand(_, None)
                | EphemSymbolModifierRegionalTokenGroup::AmbersandMut(_, None, _) => {
                    ctx.context.set_intro_implicit_self_lifetime()
                }
                EphemSymbolModifierRegionalTokenGroup::At(_, None) => {
                    ctx.context.set_intro_implicit_self_place()
                }
                _ => (),
            }
        }
        if let Some(self_value_token) = ctx.try_parse_option::<SelfValueRegionalToken>()? {
            Ok(Some(Self {
                ephem_symbol_modifier_token_group,
                self_value_token,
            }))
        } else {
            Ok(None)
        }
    }
}
