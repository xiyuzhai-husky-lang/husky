use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TypeItems {
    ast_idx_range: AstIdxRange,
}

impl TypeItems {
    pub fn ast_idx_range(&self) -> AstIdxRange {
        self.ast_idx_range
    }
}
impl IsAstChildren for TypeItems {
    const ALLOW_STMT: AstResult<()> = Err(AstError::Original(
        OriginalAstError::UnexpectedStmtInsideImplBlock,
    ));

    #[inline(always)]
    fn determine_item_kind(item_keyword_group: EntityKindKeywordGroup) -> AstResult<EntityKind> {
        let ty_item_kind = match item_keyword_group {
            EntityKindKeywordGroup::Submodule(_) => Err(OriginalAstError::UnexpectedModUnderForm)?,
            EntityKindKeywordGroup::Ritchie(ritchie_item_kind_token) => {
                TypeItemKind::MethodRitchie(ritchie_item_kind_token.ritchie_item_kind())
            }
            EntityKindKeywordGroup::AssocRitchie(_, ritchie_item_kind_token) => {
                TypeItemKind::AssocRitchie(ritchie_item_kind_token.ritchie_item_kind())
            }
            EntityKindKeywordGroup::ConceptualEntity(_) => TypeItemKind::AssocConceptual,
            EntityKindKeywordGroup::MajorType(_) => {
                Err(OriginalAstError::UnexpectedMajorTypeInsideImplBlock)?
            }
            EntityKindKeywordGroup::TypeAliasOrAssocType(_) => TypeItemKind::AssocType,
            EntityKindKeywordGroup::TypeVar(_, _) => todo!(),
            EntityKindKeywordGroup::Trait(_) => {
                Err(OriginalAstError::UnexpectedTraitInsideImplBlock)?
            }
            EntityKindKeywordGroup::Val(_) => TypeItemKind::AssocVal,
            EntityKindKeywordGroup::Memo(_) => TypeItemKind::MemoizedField,
            EntityKindKeywordGroup::Static(_) => TypeItemKind::AssocStatic,
            EntityKindKeywordGroup::Compterm(_) => TypeItemKind::AssocTermic,
        };
        Ok(EntityKind::AssocItem {
            assoc_item_kind: AssocItemKind::TypeItem(ty_item_kind),
        })
    }
}

impl<'a> TryParseOptionFromStream<AstParser<'a>> for TypeItems {
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
