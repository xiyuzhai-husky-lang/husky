use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct MajorItems;

impl IsAstChildren for MajorItems {
    const ALLOW_STMT: AstResult<()> = Ok(());

    #[inline(always)]
    fn determine_item_kind(item_keyword_group: EntityKindKeywordGroup) -> AstResult<EntityKind> {
        let module_item_kind: MajorItemKind = match item_keyword_group {
            EntityKindKeywordGroup::Submodule(_) => return Ok(EntityKind::Module),
            EntityKindKeywordGroup::Ritchie(ritchie_item_kind_token) => {
                MajorFormKind::Ritchie(ritchie_item_kind_token.ritchie_item_kind()).into()
            }
            EntityKindKeywordGroup::AssocRitchie(_, _) => {
                Err(OriginalAstError::UnexpectedStaticFnOutsideImplBlock)?
            }
            EntityKindKeywordGroup::Compterm(_) => MajorFormKind::Compterm.into(),
            EntityKindKeywordGroup::Val(_) => MajorFormKind::Val.into(),
            EntityKindKeywordGroup::Memo(_) => Err(OriginalAstError::UnexpectedMemoUnderModule)?,
            EntityKindKeywordGroup::ConceptualEntity(_) => MajorFormKind::Conceptual.into(),
            EntityKindKeywordGroup::MajorType(token) => token.type_kind().into(),
            EntityKindKeywordGroup::TypeAliasOrAssocType(_) => MajorFormKind::TypeAlias.into(),
            EntityKindKeywordGroup::TypeVar(_, _) => MajorFormKind::TypeVar.into(),
            EntityKindKeywordGroup::Trait(_) => MajorItemKind::Trait,
            EntityKindKeywordGroup::StaticMut(_, _) => MajorFormKind::StaticMut.into(),
            EntityKindKeywordGroup::StaticVar(_, _) => MajorFormKind::StaticVar.into(),
        };
        Ok(EntityKind::MajorItem {
            module_item_kind,
            connection: MajorItemConnectionKind::Connected,
        })
    }
}
