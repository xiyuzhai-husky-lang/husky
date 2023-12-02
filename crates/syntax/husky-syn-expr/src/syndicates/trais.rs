use super::*;
use husky_opr::BinaryClosedOpr;
use smallvec::smallvec;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TraitsSyndicate {
    pub colon_regional_token: ColonRegionalToken,
    pub traits_syn_expr_idx: SynExprIdx,
    pub trait_syn_expr_idxs: SmallVec<[SynExprIdx; 4]>,
}

impl<'a, 'b> TryParseOptionFromStream<SynDeclExprParser<'a>> for TraitsSyndicate {
    type Error = SynExprError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        parser: &mut SynDeclExprParser<'a>,
    ) -> SynExprResult<Option<Self>> {
        if let Some(colon_regional_token) = parser.try_parse_option::<ColonRegionalToken>()? {
            let traits_syn_expr_idx = parser.parse_expr_expected2(
                Some(ExprEnvironment::WithinBracketedParameterList(
                    SynBracket::TurboFish,
                )),
                None,
                OriginalSynExprError::ExpectedTraits,
            );
            let trait_syn_expr_idxs =
                expand_expr_into_traits(traits_syn_expr_idx, parser.syn_expr_arena());
            for &trait_syn_expr_idx in &trait_syn_expr_idxs {
                parser
                    .context_mut()
                    .add_expr_root(SynExprRootKind::Trait, trait_syn_expr_idx)
            }
            Ok(Some(TraitsSyndicate {
                colon_regional_token,
                traits_syn_expr_idx,
                trait_syn_expr_idxs,
            }))
        } else {
            Ok(None)
        }
    }
}

fn expand_expr_into_traits(
    traits_syn_expr_idx: SynExprIdx,
    syn_expr_arena: &SynExprArena,
) -> SmallVec<[SynExprIdx; 4]> {
    match syn_expr_arena[traits_syn_expr_idx] {
        SynExprData::Binary {
            lopd,
            opr: SynBinaryOpr::Closed(BinaryClosedOpr::Add),
            ropd,
            ..
        } => {
            let mut trai_syn_expr_idxs = expand_expr_into_traits(lopd, syn_expr_arena);
            trai_syn_expr_idxs.push(ropd);
            trai_syn_expr_idxs
        }
        _ => smallvec![traits_syn_expr_idx],
    }
}
