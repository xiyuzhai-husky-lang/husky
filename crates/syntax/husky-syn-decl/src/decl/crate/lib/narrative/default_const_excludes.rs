use parsec::parse_punctuated_small_list;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LibCrateSynDeclDefaultConstExcludes {
    pub default_const_excludes_ident_token: IdentRegionalToken,
    pub eq_token: EqRegionalToken,
    pub excludes: SmallVec<[LibCrateSynDeclDefaultConstExclude; 2]>,
    pub commas: SmallVec<[CommaRegionalToken; 2]>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LibCrateSynDeclDefaultConstExclude {
    expr: SynExprIdx,
}

/// # getters
impl LibCrateSynDeclDefaultConstExcludes {
    pub fn excludes(&self) -> &[LibCrateSynDeclDefaultConstExclude] {
        &self.excludes
    }
}

impl LibCrateSynDeclDefaultConstExclude {
    pub fn expr(&self) -> SynExprIdx {
        self.expr
    }
}

/// # parsers
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
        let (excludes, commas) =
            parse_punctuated_small_list::<_, _, 2, _, 2, SynNodeDeclError, SynNodeDeclError>(
                sp,
                |e| e,
            )?;
        Ok(Some(Self {
            default_const_excludes_ident_token,
            eq_token,
            excludes,
            commas,
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
