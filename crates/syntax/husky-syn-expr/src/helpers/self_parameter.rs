use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
// #[salsa::debug_with_db(db = EntitySynTreeDb)]
pub struct SelfParameterDeclPattern {
    ephem_symbol_modifier_token_group: Option<EphemSymbolModifierTokenGroup>,
    self_value_token: SelfValueToken,
}

impl SelfParameterDeclPattern {
    pub fn ephem_symbol_modifier_token_group(&self) -> Option<EphemSymbolModifierTokenGroup> {
        self.ephem_symbol_modifier_token_group
    }

    pub fn self_value_token(&self) -> SelfValueToken {
        self.self_value_token
    }
}

impl<'a, 'b> TryParseOptionFromStream<ExprParseContext<'a, 'b>> for SelfParameterDeclPattern {
    type Error = ExprError;

    // needs more testing
    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
    ) -> Result<Option<Self>, Self::Error> {
        let ephem_symbol_modifier_token_group = ctx.try_parse_option()?;
        if let Some(self_value_token) = ctx.try_parse_option::<SelfValueToken>()? {
            Ok(Some(Self {
                ephem_symbol_modifier_token_group,
                self_value_token,
            }))
        } else {
            Ok(None)
        }
    }
}
