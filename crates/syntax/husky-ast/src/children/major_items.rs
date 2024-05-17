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
            EntityKindKeywordGroup::Fn(_) => MajorFormKind::FN.into(),
            EntityKindKeywordGroup::Gn(_) => MajorFormKind::GN.into(),
            EntityKindKeywordGroup::Vn(_) => MajorFormKind::VN.into(),
            EntityKindKeywordGroup::Pn(_) => MajorFormKind::PN.into(),
            EntityKindKeywordGroup::Qn(_) => MajorFormKind::QN.into(),
            EntityKindKeywordGroup::Bn(_) => MajorFormKind::BN.into(),
            EntityKindKeywordGroup::Sn(_) => MajorFormKind::SN.into(),
            EntityKindKeywordGroup::Tn(_) => MajorFormKind::TN.into(),
            EntityKindKeywordGroup::StaticFn(_, _) => {
                Err(OriginalAstError::UnexpectedStaticFnOutsideImplBlock)?
            }
            EntityKindKeywordGroup::Const(_) => MajorFormKind::Const.into(),
            EntityKindKeywordGroup::Val(_) => MajorFormKind::Val.into(),
            EntityKindKeywordGroup::Memo(_) => Err(OriginalAstError::UnexpectedMemoUnderModule)?,
            EntityKindKeywordGroup::FormalEntity(_) => MajorFormKind::Formal.into(),
            EntityKindKeywordGroup::MajorType(token) => token.type_kind().into(),
            EntityKindKeywordGroup::AliasOrAssociateType(_) => MajorFormKind::TypeAlias.into(),
            EntityKindKeywordGroup::Trait(_) => MajorItemKind::Trait,
            EntityKindKeywordGroup::Static(_) => MajorFormKind::Static.into(),
        };
        Ok(EntityKind::MajorItem {
            module_item_kind,
            connection: MajorItemConnectionKind::Connected,
        })
    }
}
