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

impl IsAstChildren for TraitItems {
    const ALLOW_STMT: AstResult<()> = Err(AstError::Original(
        OriginalAstError::UnexpectedStmtInsideTrait,
    ));

    #[inline(always)]
    fn determine_item_kind(item_keyword_group: EntityKindKeywordGroup) -> AstResult<EntityKind> {
        let trait_item_kind = match item_keyword_group {
            EntityKindKeywordGroup::Submodule(_) => Err(OriginalAstError::UnexpectedModUnderForm)?,
            EntityKindKeywordGroup::Fn(_) => TraitItemKind::METHOD_FN,
            EntityKindKeywordGroup::Gn(_) => TraitItemKind::METHOD_GN,
            EntityKindKeywordGroup::Vn(_) => TraitItemKind::METHOD_VN,
            EntityKindKeywordGroup::Pn(_) => TraitItemKind::METHOD_PN,
            EntityKindKeywordGroup::Qn(_) => TraitItemKind::METHOD_QN,
            EntityKindKeywordGroup::Bn(_) => TraitItemKind::METHOD_BN,
            EntityKindKeywordGroup::Sn(_) => TraitItemKind::METHOD_SN,
            EntityKindKeywordGroup::Tn(_) => TraitItemKind::METHOD_TN,
            EntityKindKeywordGroup::StaticFn(_, _) => TraitItemKind::ASSOC_FN,
            EntityKindKeywordGroup::FormalEntity(_) => TraitItemKind::ASSOC_FORMAL,
            EntityKindKeywordGroup::MajorType(_) => {
                Err(OriginalAstError::UnexpectedMajorTypeInsideImplBlock)?
            }
            EntityKindKeywordGroup::AliasOrAssociateType(_) => TraitItemKind::AssocType,
            EntityKindKeywordGroup::Trait(_) => Err(OriginalAstError::UnexpectedTraitInsideTrait)?,
            EntityKindKeywordGroup::Val(_) => TraitItemKind::AssocVal,
            EntityKindKeywordGroup::Memo(_) => TraitItemKind::MemoizedField,
            EntityKindKeywordGroup::Const(_) => TraitItemKind::AssocConst,
            EntityKindKeywordGroup::Static(_) => todo!(),
        };
        let trai_item_kind = trait_item_kind;
        Ok(EntityKind::AssocItem {
            assoc_item_kind: AssocItemKind::TraitItem(trai_item_kind),
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
