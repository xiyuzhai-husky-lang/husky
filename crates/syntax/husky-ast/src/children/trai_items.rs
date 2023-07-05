use parsec::TryParseOptionFromStream;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TraitItems {
    ast_idx_range: AstIdxRange,
}

impl TraitItems {
    pub fn ast_idx_range(&self) -> AstIdxRange {
        self.ast_idx_range
    }
}

impl NormalAstChildren for TraitItems {
    const ALLOW_STMT: AstResult<()> = Err(AstError::Original(
        OriginalAstError::UnexpectedStmtInsideTrait,
    ));

    #[inline(always)]
    fn determine_entity_kind(
        entity_keyword_group: EntityKindKeywordGroup,
    ) -> AstResult<EntityKind> {
        let trai_item_kind = match entity_keyword_group {
            EntityKindKeywordGroup::Mod(_) => todo!(),
            EntityKindKeywordGroup::Fn(_) => TraitItemKind::MethodFn,
            EntityKindKeywordGroup::ConstFn(_, _) => todo!(),
            EntityKindKeywordGroup::StaticFn(_, _) => todo!(),
            EntityKindKeywordGroup::StaticConstFn(_, _, _) => todo!(),
            EntityKindKeywordGroup::Gn(_) => todo!(),
            EntityKindKeywordGroup::GeneralDef(_) => todo!(),
            EntityKindKeywordGroup::TypeEntity(_) => todo!(),
            EntityKindKeywordGroup::Type(_) => TraitItemKind::AssociatedType,
            EntityKindKeywordGroup::Trait(_) => Err(OriginalAstError::UnexpectedTraitInsideTrait)?,
            EntityKindKeywordGroup::Val(_) => todo!(),
        };
        Ok(EntityKind::AssociatedItem {
            associated_item_kind: AssociatedItemKind::TraitItem(trai_item_kind),
        })
    }
}

impl<'a> TryParseOptionFromStream<AstParser<'a>> for TraitItems {
    type Error = AstError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        parser: &mut AstParser<'a>,
    ) -> Result<Option<Self>, Self::Error> {
        Ok(parser
            .parse_normal_ast_children_indented::<Self>()
            .map(|children| Self {
                ast_idx_range: children,
            }))
    }
}
