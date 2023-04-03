use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TraitForTypeItems {
    children: AstIdxRange,
}

impl TraitForTypeItems {
    pub fn children(&self) -> AstIdxRange {
        self.children
    }
}

fn determine_trai_for_ty_item_entity_kind(
    entity_keyword_group: EntityKeywordGroup,
) -> AstResult<EntityKind> {
    Ok(match entity_keyword_group {
        EntityKeywordGroup::Mod(_) => todo!(),
        EntityKeywordGroup::Fn(_) => EntityKind::AssociatedItem {
            associated_item_kind: AssociatedItemKind::TraitForTypeItem(TraitItemKind::MethodFn),
        },
        EntityKeywordGroup::ConstFn(_, _) => todo!(),
        EntityKeywordGroup::StaticFn(_, _) => todo!(),
        EntityKeywordGroup::StaticConstFn(_, _, _) => todo!(),
        EntityKeywordGroup::Gn(_) => todo!(),
        EntityKeywordGroup::GeneralDef(_) => todo!(),
        EntityKeywordGroup::TypeEntity(_) => todo!(),
        EntityKeywordGroup::Type(_) => todo!(),
        EntityKeywordGroup::Trait(_) => todo!(),
        EntityKeywordGroup::Visual(_) => todo!(),
        EntityKeywordGroup::Val(_) => todo!(),
        EntityKeywordGroup::Memo(_) => todo!(),
    })
}
