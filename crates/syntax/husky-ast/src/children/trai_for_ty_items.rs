use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TraitForTypeItems {
    ast_idx_range: AstIdxRange,
}

impl TraitForTypeItems {
    pub fn ast_idx_range(&self) -> AstIdxRange {
        self.ast_idx_range
    }
}
impl NormalAstChildren for TraitForTypeItems {
    const ALLOW_STMT: AstResult<()> = Err(AstError::Original(
        OriginalAstError::UnexpectedStmtInsideImplBlock,
    ));

    #[inline(always)]
    fn determine_entity_kind(
        entity_keyword_group: EntityKindKeywordGroup,
    ) -> AstResult<EntityKind> {
        Ok(match entity_keyword_group {
            EntityKindKeywordGroup::Mod(_) => todo!(),
            EntityKindKeywordGroup::Fn(_) => EntityKind::AssociatedItem {
                associated_item_kind: AssociatedItemKind::TraitForTypeItem(TraitItemKind::MethodFn),
            },
            EntityKindKeywordGroup::ConstFn(_, _) => todo!(),
            EntityKindKeywordGroup::StaticFn(_, _) => todo!(),
            EntityKindKeywordGroup::StaticConstFn(_, _, _) => todo!(),
            EntityKindKeywordGroup::Gn(_) => todo!(),
            EntityKindKeywordGroup::GeneralDef(_) => todo!(),
            EntityKindKeywordGroup::TypeEntity(_) => todo!(),
            EntityKindKeywordGroup::Type(_) => todo!(),
            EntityKindKeywordGroup::Trait(_) => todo!(),
            EntityKindKeywordGroup::Visual(_) => todo!(),
            EntityKindKeywordGroup::Val(_) => todo!(),
            EntityKindKeywordGroup::Memo(_) => todo!(),
        })
    }
}

impl<'a> ParseFromStream<AstParser<'a>> for TraitForTypeItems {
    type Error = AstError;

    fn parse_from_without_guaranteed_rollback(
        parser: &mut AstParser<'a>,
    ) -> Result<Option<Self>, Self::Error> {
        Ok(parser
            .parse_normal_ast_children_indented::<Self>()
            .map(|children| Self {
                ast_idx_range: children,
            }))
    }
}
