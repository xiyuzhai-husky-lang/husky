use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct LibCrateSynDeclDefaultConstExcludes {
    pub default_const_excludes_ident_token: IdentRegionalToken,
    pub eq_token: EqRegionalToken,
    pub excludes: PunctuatedSmallList<
        LibCrateSynDeclDefaultConstExclude,
        CommaRegionalToken,
        SynNodeDeclError,
        false,
        4,
    >,
}

#[derive(Debug, PartialEq, Eq)]
pub struct LibCrateSynDeclDefaultConstExclude {
    expr: SynExprIdx,
}

impl<'db, 'a> TryParseOptionFromStream<ProducedSynExprParser<'db, 'a>>
    for LibCrateSynDeclDefaultConstExcludes
{
    type Error = SynNodeDeclError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        sp: &mut ProducedSynExprParser<'db, 'a>,
    ) -> Result<Option<Self>, Self::Error> {
        let db = sp.db();
        let Some(default_const_excludes_ident_token) =
            sp.try_parse_option::<IdentRegionalToken>()?
        else {
            return Ok(None);
        };
        if default_const_excludes_ident_token.ident().data(db) != "default_const_excludes" {
            return Ok(None);
        }
        let eq_token = sp.try_parse_expected::<EqRegionalToken, _>(
            OriginalSynNodeDeclError::ExpectedEqTokenForLibCrateDefaultConstExcludes,
        )?;
        let excludes = sp.try_parse()?;
        Ok(Some(Self {
            default_const_excludes_ident_token,
            eq_token,
            excludes,
        }))
    }
}

impl<'db, 'a> TryParseOptionFromStream<ProducedSynExprParser<'db, 'a>>
    for LibCrateSynDeclDefaultConstExclude
{
    type Error = SynNodeDeclError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        sp: &mut ProducedSynExprParser<'db, 'a>,
    ) -> Result<Option<Self>, Self::Error> {
        let Some(expr) = sp.parse_expr_root(None, SynExprRootKind::DefaultConstExclude) else {
            return Ok(None);
        };
        Ok(Some(Self { expr }))
    }
}
