use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct MajorItems;

impl NormalAstChildren for MajorItems {
    const ALLOW_STMT: AstResult<()> = Err(AstError::Original(
        OriginalAstError::UnexpectedStmtInsideModule,
    ));

    #[inline(always)]
    fn determine_item_kind(item_keyword_group: EntityKindKeywordGroup) -> AstResult<EntityKind> {
        let module_item_kind: MajorItemKind = match item_keyword_group {
            EntityKindKeywordGroup::Mod(_) => return Ok(EntityKind::Module),
            EntityKindKeywordGroup::Fn(_) => FugitiveKind::Fn.into(),
            EntityKindKeywordGroup::ConstFn(_, _) => todo!(),
            EntityKindKeywordGroup::StaticFn(_, _) => todo!(),
            EntityKindKeywordGroup::StaticConstFn(_, _, _) => todo!(),
            EntityKindKeywordGroup::Val(_) => FugitiveKind::Val.into(),
            EntityKindKeywordGroup::Gn(_) => FugitiveKind::Gn.into(),
            EntityKindKeywordGroup::GeneralDef(_) => todo!(),
            EntityKindKeywordGroup::TypeEntity(token) => token.type_kind().into(),
            EntityKindKeywordGroup::Type(_) => FugitiveKind::AliasType.into(),
            EntityKindKeywordGroup::Trait(_) => MajorItemKind::Trait,
        };
        Ok(EntityKind::MajorItem {
            module_item_kind,
            connection: MajorItemConnectionKind::Connected,
        })
    }
}
