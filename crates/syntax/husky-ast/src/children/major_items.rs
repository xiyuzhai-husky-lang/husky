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
            EntityKindKeywordGroup::Fn(_) => MajorFugitiveKind::FN.into(),
            EntityKindKeywordGroup::Gn(_) => MajorFugitiveKind::GN.into(),
            EntityKindKeywordGroup::Vn(_) => MajorFugitiveKind::VN.into(),
            EntityKindKeywordGroup::Pn(_) => MajorFugitiveKind::PN.into(),
            EntityKindKeywordGroup::Qn(_) => MajorFugitiveKind::QN.into(),
            EntityKindKeywordGroup::Tn(_) => MajorFugitiveKind::TN.into(),
            EntityKindKeywordGroup::StaticFn(_, _) => {
                Err(OriginalAstError::UnexpectedStaticFnOutsideImplBlock)?
            }
            EntityKindKeywordGroup::Const(_) => MajorFugitiveKind::Const.into(),
            EntityKindKeywordGroup::Val(_) => MajorFugitiveKind::Val.into(),
            EntityKindKeywordGroup::Memo(_) => Err(OriginalAstError::UnexpectedMemoUnderModule)?,
            EntityKindKeywordGroup::FormalEntity(_) => MajorFugitiveKind::Formal.into(),
            EntityKindKeywordGroup::MajorType(token) => token.type_kind().into(),
            EntityKindKeywordGroup::AliasOrAssociateType(_) => MajorFugitiveKind::TypeAlias.into(),
            EntityKindKeywordGroup::Trait(_) => MajorItemKind::Trait,
        };
        Ok(EntityKind::MajorItem {
            module_item_kind,
            connection: MajorItemConnectionKind::Connected,
        })
    }
}
