use crate::{
    ClosureSynPatternExprRoot, CurrentSynSymbolData, CurrentSynSymbolEntry,
    CurrentSynSymbolIdxRange, ExprEnvironment, IsSynExprContext, OriginalSynExprError,
    SynExprError, SynExprIdx, SynExprParser, SynExprResult, SynExprRootKind,
    SyndicateTypeConstraint,
};
use husky_regional_token::ColonRegionalToken;
use husky_token_data::delimiter::Delimiter;
use parsec::TryParseOptionFromStream;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClosureParameterSyndicate {
    Simple {
        syn_pattern_root: ClosureSynPatternExprRoot,
        variables: CurrentSynSymbolIdxRange,
        ty: Option<(ColonRegionalToken, SynExprIdx)>,
    },
}

impl<'a, C> TryParseOptionFromStream<SynExprParser<'a, C>> for ClosureParameterSyndicate
where
    C: IsSynExprContext<'a>,
{
    type Error = SynExprError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut SynExprParser<'a, C>,
    ) -> SynExprResult<Option<Self>> {
        use parsec::{HasStreamState, IsStreamParser};

        if let Some(syn_pattern_root) = ctx.try_parse_option::<ClosureSynPatternExprRoot>()? {
            let symbols = ctx
                .pattern_expr_region()
                .pattern_expr_symbols(syn_pattern_root.syn_pattern_expr_idx());
            let access_start = ctx.state().next_regional_token_idx();
            let variables = symbols
                .iter()
                .map(|&(ident, pattern_symbol_idx)| {
                    CurrentSynSymbolEntry::new(
                        ctx.pattern_expr_region(),
                        access_start,
                        None,
                        CurrentSynSymbolData::SimpleClosureParameter {
                            ident,
                            pattern_symbol_idx,
                        },
                    )
                })
                .collect::<Vec<_>>();
            let colon: Option<ColonRegionalToken> = ctx.try_parse_option()?;
            let ty = colon.map(|colon| {
                let ty_expr_idx = ctx.parse_expr_expected2(
                    Some(ExprEnvironment::WithinDelimiteredParameterList(
                        Delimiter::Vert,
                    )),
                    SynExprRootKind::ExplicitParameterType,
                    OriginalSynExprError::ExpectedParameterType,
                );
                (colon, ty_expr_idx)
            });
            let variables = ctx.define_symbols(
                variables,
                ty.map(
                    |(_, ty_expr_idx)| SyndicateTypeConstraint::SimpleClosureParameter {
                        syn_pattern_root,
                        ty: ty_expr_idx,
                    },
                ),
            );
            Ok(Some(ClosureParameterSyndicate::Simple {
                syn_pattern_root,
                variables,
                ty,
            }))
        } else {
            Ok(None)
        }
    }
}
