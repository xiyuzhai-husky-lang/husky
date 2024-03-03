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
                Err(OriginalAstError::UnexpectedModUnderFugitive)?
            }
            EntityKindKeywordGroup::Fn(_) => MajorFugitiveKind::FN.into(),
            EntityKindKeywordGroup::StaticFn(_, _) => {
                Err(OriginalAstError::UnexpectedStaticFnOutsideImplBlock)?
            }
            EntityKindKeywordGroup::Gn(_) => MajorFugitiveKind::GN.into(),
            EntityKindKeywordGroup::Vn(_) => MajorFugitiveKind::VN.into(),
            EntityKindKeywordGroup::Pn(_) => MajorFugitiveKind::PN.into(),
            EntityKindKeywordGroup::Qn(_) => MajorFugitiveKind::QN.into(),
            EntityKindKeywordGroup::Tn(_) => MajorFugitiveKind::TN.into(),
            EntityKindKeywordGroup::FormalEntity(_) => MajorFugitiveKind::Formal.into(),
            EntityKindKeywordGroup::MajorType(token) => token.type_kind().into(),
            EntityKindKeywordGroup::AliasOrAssociateType(_) => MajorFugitiveKind::TypeAlias.into(),
            EntityKindKeywordGroup::Trait(_) => MajorItemKind::Trait,
            EntityKindKeywordGroup::Val(_) => MajorFugitiveKind::Val.into(),
            EntityKindKeywordGroup::Const(_) => MajorFugitiveKind::Const.into(),
            EntityKindKeywordGroup::Memo(_) => Err(OriginalAstError::UnexpectedMemoUnderFugitive)?,
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
