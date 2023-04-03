use parsec::ParseFromStream;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TraitItems {
    children: AstIdxRange,
}

impl TraitItems {
    pub fn children(&self) -> AstIdxRange {
        self.children
    }
}

impl<'a> ParseFromStream<AstParser<'a>> for TraitItems {
    type Error = AstError;

    fn parse_from_without_guaranteed_rollback(
        parser: &mut AstParser<'a>,
    ) -> Result<Option<Self>, Self::Error> {
        // parser.parse_indented_children(|| todo!(), todo!());
        Ok(todo!())
    }
}

fn determine_trai_item_entity_kind(
    entity_keyword_group: EntityKeywordGroup,
) -> AstResult<EntityKind> {
    let trai_item_kind = match entity_keyword_group {
        EntityKeywordGroup::Mod(_) => todo!(),
        EntityKeywordGroup::Fn(_) => TraitItemKind::MethodFn,
        EntityKeywordGroup::ConstFn(_, _) => todo!(),
        EntityKeywordGroup::StaticFn(_, _) => todo!(),
        EntityKeywordGroup::StaticConstFn(_, _, _) => todo!(),
        EntityKeywordGroup::Gn(_) => todo!(),
        EntityKeywordGroup::GeneralDef(_) => todo!(),
        EntityKeywordGroup::TypeEntity(_) => todo!(),
        EntityKeywordGroup::Type(_) => TraitItemKind::AssociatedType,
        EntityKeywordGroup::Trait(_) => Err(OriginalAstError::UnexpectedTraitInsideTrait)?,
        EntityKeywordGroup::Visual(_) => todo!(),
        EntityKeywordGroup::Val(_) => todo!(),
        EntityKeywordGroup::Memo(_) => todo!(),
    };
    Ok(EntityKind::AssociatedItem {
        associated_item_kind: AssociatedItemKind::TraitItem(trai_item_kind),
    })
}
