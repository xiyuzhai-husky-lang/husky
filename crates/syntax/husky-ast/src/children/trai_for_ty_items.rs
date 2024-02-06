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

impl IsAstChildren for TraitForTypeItems {
    const ALLOW_STMT: AstResult<()> = Err(AstError::Original(
        OriginalAstError::UnexpectedStmtInsideImplBlock,
    ));

    #[inline(always)]
    fn determine_item_kind(item_keyword_group: EntityKindKeywordGroup) -> AstResult<EntityKind> {
        let trait_item_kind = match item_keyword_group {
            EntityKindKeywordGroup::Submodule(_) => {
                Err(OriginalAstError::UnexpectedModUnderFugitive)?
            }
            EntityKindKeywordGroup::FugitiveFn(_) => TraitItemKind::MethodFn,
            EntityKindKeywordGroup::StaticFn(_, _) => TraitItemKind::AssocFunctionFn,
            EntityKindKeywordGroup::Gn(_) => TraitItemKind::AssocFunctionGn,
            EntityKindKeywordGroup::FormalEntity(_) => TraitItemKind::AssocFormal,
            EntityKindKeywordGroup::MajorType(_) => {
                Err(OriginalAstError::UnexpectedMajorTypeInsideImplBlock)?
            }
            EntityKindKeywordGroup::AliasOrAssociateType(_) => TraitItemKind::AssocType,
            EntityKindKeywordGroup::Trait(_) => Err(OriginalAstError::UnexpectedTraitInsideTrait)?,
            EntityKindKeywordGroup::Val(_) => TraitItemKind::AssocVal,
            EntityKindKeywordGroup::Memo(_) => TraitItemKind::MemoizedField,
            EntityKindKeywordGroup::Const(_) => todo!(),
        };
        Ok(EntityKind::AssocItem {
            associated_item_kind: AssocItemKind::TraitForTypeItem(trait_item_kind),
        })
    }
}

impl<'a> TryParseOptionFromStream<AstParser<'a>> for TraitForTypeItems {
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
