use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct FormBody {
    ast_idx_range: AstIdxRange,
}

/// # getters
impl FormBody {
    pub fn ast_idx_range(&self) -> AstIdxRange {
        self.ast_idx_range
    }
}

impl IsAstChildren for FormBody {
    const ALLOW_STMT: AstResult<()> = Ok(());

    #[inline(always)]
    fn determine_item_kind(item_keyword_group: EntityKindKeywordGroup) -> AstResult<EntityKind> {
        let module_item_kind = match item_keyword_group {
            EntityKindKeywordGroup::Submodule(_) => Err(OriginalAstError::UnexpectedModUnderForm)?,
            EntityKindKeywordGroup::Fn(_) => MajorFormKind::FN.into(),
            EntityKindKeywordGroup::StaticFn(_, _) => {
                Err(OriginalAstError::UnexpectedStaticFnOutsideImplBlock)?
            }
            EntityKindKeywordGroup::Gn(_) => MajorFormKind::GN.into(),
            EntityKindKeywordGroup::Vn(_) => MajorFormKind::VN.into(),
            EntityKindKeywordGroup::Pn(_) => MajorFormKind::PN.into(),
            EntityKindKeywordGroup::Qn(_) => MajorFormKind::QN.into(),
            EntityKindKeywordGroup::Tn(_) => MajorFormKind::TN.into(),
            EntityKindKeywordGroup::FormalEntity(_) => MajorFormKind::Formal.into(),
            EntityKindKeywordGroup::MajorType(token) => token.type_kind().into(),
            EntityKindKeywordGroup::AliasOrAssociateType(_) => MajorFormKind::TypeAlias.into(),
            EntityKindKeywordGroup::Trait(_) => MajorItemKind::Trait,
            EntityKindKeywordGroup::Val(_) => MajorFormKind::Val.into(),
            EntityKindKeywordGroup::Const(_) => MajorFormKind::Const.into(),
            EntityKindKeywordGroup::Static(_) => MajorFormKind::Static.into(),
            EntityKindKeywordGroup::Memo(_) => Err(OriginalAstError::UnexpectedMemoUnderForm)?,
        };
        Ok(EntityKind::MajorItem {
            module_item_kind,
            connection: MajorItemConnectionKind::Disconnected,
        })
    }
}

impl<'a> TryParseOptionFromStream<AstParser<'a>> for FormBody {
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
