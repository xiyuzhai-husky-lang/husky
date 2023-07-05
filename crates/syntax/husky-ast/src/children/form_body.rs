use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct FugitiveBody {
    ast_idx_range: AstIdxRange,
}

impl FugitiveBody {
    pub fn ast_idx_range(&self) -> AstIdxRange {
        self.ast_idx_range
    }
}

impl NormalAstChildren for FugitiveBody {
    const ALLOW_STMT: AstResult<()> = Ok(());

    #[inline(always)]
    fn determine_entity_kind(
        entity_keyword_group: EntityKindKeywordGroup,
    ) -> AstResult<EntityKind> {
        let module_item_kind = match entity_keyword_group {
            EntityKindKeywordGroup::Mod(_) => Err(OriginalAstError::UnexpectedModInsideForm)?,
            EntityKindKeywordGroup::Fn(_) => FugitiveKind::Fn.into(),
            EntityKindKeywordGroup::ConstFn(_, _) => todo!(),
            EntityKindKeywordGroup::StaticFn(_, _) => todo!(),
            EntityKindKeywordGroup::StaticConstFn(_, _, _) => todo!(),
            EntityKindKeywordGroup::Gn(_) => todo!(),
            EntityKindKeywordGroup::GeneralDef(_) => todo!(),
            EntityKindKeywordGroup::TypeEntity(token) => token.type_kind().into(),
            EntityKindKeywordGroup::Type(_) => todo!(),
            EntityKindKeywordGroup::Trait(_) => todo!(),
            EntityKindKeywordGroup::Val(_) => FugitiveKind::Val.into(),
        };
        Ok(EntityKind::ModuleItem {
            module_item_kind,
            connection: ModuleItemConnectionKind::Disconnected,
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
