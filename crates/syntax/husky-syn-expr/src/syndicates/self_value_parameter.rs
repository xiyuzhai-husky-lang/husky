use husky_entity_tree::region_path::SynNodeRegionPath;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
// #[salsa::derive_debug_with_db]
pub struct SelfValueParameterSyndicate {
    ephem_symbol_modifier_token_verse: Option<EphemSymbolModifierRegionalTokens>,
    self_value_token: SelfValueRegionalToken,
}

impl SelfValueParameterSyndicate {
    pub fn ephem_symbol_modifier_token_verse(&self) -> Option<EphemSymbolModifierRegionalTokens> {
        self.ephem_symbol_modifier_token_verse
    }

    pub fn self_value_token(&self) -> SelfValueRegionalToken {
        self.self_value_token
    }
}

impl<'a, 'b> TryParseOptionFromStream<StandaloneSynExprParser<'a, SynNodeRegionPath>>
    for SelfValueParameterSyndicate
{
    type Error = SynExprError;

    // needs more testing
    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut StandaloneSynExprParser<'a, SynNodeRegionPath>,
    ) -> Result<Option<Self>, Self::Error> {
        let ephem_symbol_modifier_token_verse = ctx.try_parse_option()?;
        let Some(self_value_token) = ctx.try_parse_option::<SelfValueRegionalToken>()? else {
            return Ok(None);
        };
        if let Some(ephem_symbol_modifier_token_verse) = ephem_symbol_modifier_token_verse {
            match ephem_symbol_modifier_token_verse {
                EphemSymbolModifierRegionalTokens::Ambersand(_, None)
                | EphemSymbolModifierRegionalTokens::AmbersandMut(_, None, _) => {
                    ctx.context.set_has_self_lifetime()
                }
                EphemSymbolModifierRegionalTokens::At(_, None) => ctx.context.set_has_self_place(),
                _ => (),
            }
        }
        Ok(Some(Self {
            ephem_symbol_modifier_token_verse,
            self_value_token,
        }))
    }
}
