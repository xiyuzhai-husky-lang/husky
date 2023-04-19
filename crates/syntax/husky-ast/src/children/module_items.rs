use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ModuleItems;

impl NormalAstChildren for ModuleItems {
    const ALLOW_STMT: AstResult<()> = Err(AstError::Original(
        OriginalAstError::UnexpectedStmtInsideModule,
    ));

    #[inline(always)]
    fn determine_entity_kind(
        entity_keyword_group: EntityKindKeywordGroup,
    ) -> AstResult<EntityKind> {
        let module_item_kind: ModuleItemKind = match entity_keyword_group {
            EntityKindKeywordGroup::Mod(_) => return Ok(EntityKind::Module),
            EntityKindKeywordGroup::Fn(_) => FugitiveKind::Fn.into(),
            EntityKindKeywordGroup::ConstFn(_, _) => todo!(),
            EntityKindKeywordGroup::StaticFn(_, _) => todo!(),
            EntityKindKeywordGroup::StaticConstFn(_, _, _) => todo!(),
            EntityKindKeywordGroup::Val(_) => FugitiveKind::Val.into(),
            EntityKindKeywordGroup::Gn(_) => FugitiveKind::Gn.into(),
            EntityKindKeywordGroup::GeneralDef(_) => todo!(),
            EntityKindKeywordGroup::TypeEntity(token) => token.type_kind().into(),
            EntityKindKeywordGroup::Type(_) => FugitiveKind::Type.into(),
            EntityKindKeywordGroup::Trait(_) => ModuleItemKind::Trait,
            EntityKindKeywordGroup::Visual(_) => todo!(),
            EntityKindKeywordGroup::Memo(_) => todo!(),
        };
        Ok(EntityKind::ModuleItem {
            module_item_kind,
            connection: ModuleItemConnectionKind::Connected,
        })
    }
}
