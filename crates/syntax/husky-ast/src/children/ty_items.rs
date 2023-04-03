use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TypeItems {
    children: AstIdxRange,
}

impl TypeItems {
    pub fn children(&self) -> AstIdxRange {
        self.children
    }
}

fn determine_ty_item_entity_kind(
    entity_keyword_group: EntityKeywordGroup,
) -> AstResult<EntityKind> {
    let ty_item_kind = match entity_keyword_group {
        EntityKeywordGroup::Mod(_) => todo!(),
        EntityKeywordGroup::Fn(_) => TypeItemKind::MethodFn,
        EntityKeywordGroup::ConstFn(_, _) => todo!(),
        EntityKeywordGroup::StaticFn(_, _) => TypeItemKind::AssociatedFn,
        EntityKeywordGroup::StaticConstFn(_, _, _) => todo!(),
        EntityKeywordGroup::Gn(_) => todo!(),
        EntityKeywordGroup::GeneralDef(_) => todo!(),
        EntityKeywordGroup::TypeEntity(_) => {
            Err(OriginalAstError::UnexpectedTypeDefnInsideTypeImplBlock)?
        }
        EntityKeywordGroup::Type(_) => todo!(),
        EntityKeywordGroup::Trait(_) => todo!(),
        EntityKeywordGroup::Visual(_) => todo!(),
        EntityKeywordGroup::Val(_) => TypeItemKind::AssociatedVar,
        EntityKeywordGroup::Memo(_) => TypeItemKind::Memo,
    };
    Ok(EntityKind::AssociatedItem {
        associated_item_kind: AssociatedItemKind::TypeItem(ty_item_kind),
    })
}
