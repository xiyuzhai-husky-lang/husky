use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct FormBody {
    children: AstIdxRange,
}

impl FormBody {
    pub fn children(&self) -> AstIdxRange {
        self.children
    }
}

fn determine_form_item_entity_kind(
    entity_keyword_group: EntityKeywordGroup,
) -> AstResult<EntityKind> {
    let module_item_kind = match entity_keyword_group {
        EntityKeywordGroup::Mod(_) => Err(OriginalAstError::UnexpectedModInsideForm)?,
        EntityKeywordGroup::Fn(_) => FormKind::Fn.into(),
        EntityKeywordGroup::ConstFn(_, _) => todo!(),
        EntityKeywordGroup::StaticFn(_, _) => todo!(),
        EntityKeywordGroup::StaticConstFn(_, _, _) => todo!(),
        EntityKeywordGroup::Gn(_) => todo!(),
        EntityKeywordGroup::GeneralDef(_) => todo!(),
        EntityKeywordGroup::TypeEntity(token) => token.type_kind().into(),
        EntityKeywordGroup::Type(_) => todo!(),
        EntityKeywordGroup::Trait(_) => todo!(),
        EntityKeywordGroup::Visual(_) => todo!(),
        EntityKeywordGroup::Val(_) => FormKind::Val.into(),
        EntityKeywordGroup::Memo(_) => Err(OriginalAstError::UnexpectedMemoFieldInsideForm)?,
    };
    Ok(EntityKind::ModuleItem {
        module_item_kind,
        connection: ModuleItemConnectionKind::Disconnected,
    })
}
