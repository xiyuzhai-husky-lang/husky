use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct MajorItems;

impl IsAstChildren for MajorItems {
    const ALLOW_STMT: AstResult<()> = Err(AstError::Original(
        OriginalAstError::UnexpectedStmtUnderModule,
    ));

    #[inline(always)]
    fn determine_item_kind(item_keyword_group: EntityKindKeywordGroup) -> AstResult<EntityKind> {
        let module_item_kind: MajorItemKind = match item_keyword_group {
            EntityKindKeywordGroup::Submodule(_) => return Ok(EntityKind::Module),
            EntityKindKeywordGroup::FugitiveFn(_) => FugitiveKind::FunctionFn.into(),
            EntityKindKeywordGroup::StaticFn(_, _) => {
                Err(OriginalAstError::UnexpectedStaticFnOutsideImplBlock)?
            }
            EntityKindKeywordGroup::Const(_) => FugitiveKind::Const.into(),
            EntityKindKeywordGroup::Val(_) => FugitiveKind::Val.into(),
            EntityKindKeywordGroup::Memo(_) => Err(OriginalAstError::UnexpectedMemoUnderModule)?,
            EntityKindKeywordGroup::Gn(_) => FugitiveKind::FunctionGn.into(),
            EntityKindKeywordGroup::FormalEntity(_) => FugitiveKind::Formal.into(),
            EntityKindKeywordGroup::MajorType(token) => token.type_kind().into(),
            EntityKindKeywordGroup::AliasOrAssociateType(_) => FugitiveKind::TypeAlias.into(),
            EntityKindKeywordGroup::Trait(_) => MajorItemKind::Trait,
        };
        Ok(EntityKind::MajorItem {
            module_item_kind,
            connection: MajorItemConnectionKind::Connected,
        })
    }
}
