#[path = "narrative/default_const_excludes.rs"]
pub mod default_const_excludes;

use self::default_const_excludes::LibCrateSynDeclDefaultConstExcludes;
use super::*;
use husky_entity_tree::region_path::SynNodeRegionPath;

#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum LibCrateSynDeclNarrative {
    DefaultConstExcludes(LibCrateSynDeclDefaultConstExcludes),
}

impl<'db, 'a> TryParseOptionFromStream<ProducedSynExprParser<'db, 'a, SynNodeRegionPath>>
    for LibCrateSynDeclNarrative
{
    type Error = SynNodeDeclError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        sp: &mut ProducedSynExprParser<'db, 'a, SynNodeRegionPath>,
    ) -> Result<Option<Self>, Self::Error> {
        if let Some(default_const_excludes) =
            sp.try_parse_option::<LibCrateSynDeclDefaultConstExcludes>()?
        {
            Ok(Some(default_const_excludes.into()))
        } else {
            Ok(None)
        }
    }
}

pub(crate) fn parse_narrative<'db, 'a>(
    mut parser: ProducedSynExprParser<'db, 'a, SynNodeRegionPath>,
    narrate_token: NarrateRegionalToken,
) -> SynNodeDeclResult<LibCrateSynDeclItem> {
    let narrative =
        parser.try_parse_expected(OriginalSynNodeDeclError::ExpectedCrateDeclNarrative)?;
    Ok(LibCrateSynDeclItem::Narrative {
        narrate_token,
        narrative,
    })
}
