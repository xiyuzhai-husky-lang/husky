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
impl NormalAstChildren for TypeItems {
    const ALLOW_STMT: AstResult<()> = Err(AstError::Original(
        OriginalAstError::UnexpectedStmtInsideImplBlock,
    ));

    #[inline(always)]
    fn determine_entity_kind(
        entity_keyword_group: EntityKindKeywordGroup,
    ) -> AstResult<EntityKind> {
        let ty_item_kind = match entity_keyword_group {
            EntityKindKeywordGroup::Mod(_) => todo!(),
            EntityKindKeywordGroup::Fn(_) => TypeItemKind::MethodFn,
            EntityKindKeywordGroup::ConstFn(_, _) => todo!(),
            EntityKindKeywordGroup::StaticFn(_, _) => TypeItemKind::AssociatedFn,
            EntityKindKeywordGroup::StaticConstFn(_, _, _) => todo!(),
            EntityKindKeywordGroup::Gn(_) => todo!(),
            EntityKindKeywordGroup::GeneralDef(_) => todo!(),
            EntityKindKeywordGroup::TypeEntity(_) => {
                Err(OriginalAstError::UnexpectedTypeDefnInsideTypeImplBlock)?
            }
            EntityKindKeywordGroup::Type(_) => todo!(),
            EntityKindKeywordGroup::Trait(_) => todo!(),
            EntityKindKeywordGroup::Val(_) => TypeItemKind::MemoizedField,
        };
        Ok(EntityKind::AssociatedItem {
            associated_item_kind: AssociatedItemKind::TypeItem(ty_item_kind),
        })
    }
}

impl<'a> TryParseOptionalFromStream<AstParser<'a>> for TypeItems {
    type Error = AstError;

    fn try_parse_optional_from_without_guaranteed_rollback(
        parser: &mut AstParser<'a>,
    ) -> Result<Option<Self>, Self::Error> {
        Ok(parser
            .parse_normal_ast_children_indented::<Self>()
            .map(|children| Self {
                ast_idx_range: children,
            }))
    }
}
