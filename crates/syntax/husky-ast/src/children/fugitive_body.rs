use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct FugitiveBody {
    ast_idx_range: AstIdxRange,
}

/// # getters
impl FugitiveBody {
    pub fn ast_idx_range(&self) -> AstIdxRange {
        self.ast_idx_range
    }
}

impl IsAstChildren for FugitiveBody {
    const ALLOW_STMT: AstResult<()> = Ok(());

    #[inline(always)]
    fn determine_item_kind(item_keyword_group: EntityKindKeywordGroup) -> AstResult<EntityKind> {
        let module_item_kind = match item_keyword_group {
            EntityKindKeywordGroup::Submodule(_) => {
                Err(OriginalAstError::UnexpectedModInsideModuleItem)?
            }
            EntityKindKeywordGroup::FugitiveFn(_) => FugitiveKind::FunctionFn.into(),
            EntityKindKeywordGroup::StaticFn(_, _) => {
                Err(OriginalAstError::UnexpectedStaticFnOutsideImplBlock)?
            }
            EntityKindKeywordGroup::Gn(_) => FugitiveKind::FunctionGn.into(),
            EntityKindKeywordGroup::FormalEntity(_) => FugitiveKind::Formal.into(),
            EntityKindKeywordGroup::MajorType(token) => token.type_kind().into(),
            EntityKindKeywordGroup::AliasOrAssociateType(_) => FugitiveKind::AliasType.into(),
            EntityKindKeywordGroup::Trait(_) => MajorItemKind::Trait,
            EntityKindKeywordGroup::Val(_) => FugitiveKind::Val.into(),
            EntityKindKeywordGroup::Const(_) => FugitiveKind::Const.into(),
        };
        Ok(EntityKind::MajorItem {
            module_item_kind,
            connection: MajorItemConnectionKind::Disconnected,
        })
    }
}

impl<'a> TryParseOptionFromStream<AstParser<'a>> for FugitiveBody {
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
