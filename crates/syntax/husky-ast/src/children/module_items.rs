use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ModuleItems {
    children: AstIdxRange,
}

impl IndentedChildren for ModuleItems {
    const ALLOW_STMT: AstResult<()> = Err(AstError::Original(
        OriginalAstError::UnexpectedStmtInsideModule,
    ));

    #[inline(always)]
    fn determine_entity_kind(entity_keyword_group: EntityKeywordGroup) -> AstResult<EntityKind> {
        let module_item_kind: ModuleItemKind = match entity_keyword_group {
            EntityKeywordGroup::Mod(_) => return Ok(EntityKind::Module),
            EntityKeywordGroup::Fn(_) => FormKind::Fn.into(),
            EntityKeywordGroup::ConstFn(_, _) => todo!(),
            EntityKeywordGroup::StaticFn(_, _) => todo!(),
            EntityKeywordGroup::StaticConstFn(_, _, _) => todo!(),
            EntityKeywordGroup::Val(_) => FormKind::Val.into(),
            EntityKeywordGroup::Gn(_) => FormKind::Gn.into(),
            EntityKeywordGroup::GeneralDef(_) => todo!(),
            EntityKeywordGroup::TypeEntity(token) => token.type_kind().into(),
            EntityKeywordGroup::Type(_) => FormKind::TypeAlias.into(),
            EntityKeywordGroup::Trait(_) => ModuleItemKind::Trait,
            EntityKeywordGroup::Visual(_) => todo!(),
            EntityKeywordGroup::Memo(_) => todo!(),
        };
        Ok(EntityKind::ModuleItem {
            module_item_kind,
            connection: ModuleItemConnectionKind::Connected,
        })
    }
}

impl ModuleItems {
    pub fn children(&self) -> AstIdxRange {
        self.children
    }
}

fn determine_module_item_entity_kind(
    entity_keyword_group: EntityKeywordGroup,
) -> AstResult<EntityKind> {
    let module_item_kind: ModuleItemKind = match entity_keyword_group {
        EntityKeywordGroup::Mod(_) => return Ok(EntityKind::Module),
        EntityKeywordGroup::Fn(_) => FormKind::Fn.into(),
        EntityKeywordGroup::ConstFn(_, _) => todo!(),
        EntityKeywordGroup::StaticFn(_, _) => todo!(),
        EntityKeywordGroup::StaticConstFn(_, _, _) => todo!(),
        EntityKeywordGroup::Val(_) => FormKind::Val.into(),
        EntityKeywordGroup::Gn(_) => FormKind::Gn.into(),
        EntityKeywordGroup::GeneralDef(_) => todo!(),
        EntityKeywordGroup::TypeEntity(token) => token.type_kind().into(),
        EntityKeywordGroup::Type(_) => FormKind::TypeAlias.into(),
        EntityKeywordGroup::Trait(_) => ModuleItemKind::Trait,
        EntityKeywordGroup::Visual(_) => todo!(),
        EntityKeywordGroup::Memo(_) => todo!(),
    };
    Ok(EntityKind::ModuleItem {
        module_item_kind,
        connection: ModuleItemConnectionKind::Connected,
    })
}
