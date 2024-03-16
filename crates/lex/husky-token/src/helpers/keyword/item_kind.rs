use super::*;

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityKindKeywordGroup {
    /// todo: remove mod
    Submodule(ModToken),
    /// `fn`
    Fn(FnToken),
    /// `gn`
    Gn(GnToken),
    /// `vn`
    Vn(VnToken),
    /// `pn`
    Pn(PnToken),
    /// `qn`
    Qn(QnToken),
    /// `vn`
    Tn(TnToken),
    /// `static fn`
    StaticFn(StaticToken, FnToken),
    /// `val`
    Ki(ValToken),
    /// `memo`
    Memo(MemoToken),
    /// husky will have the capacities of theorem proving
    FormalEntity(FormalEntityToken),
    /// type defined as a major entity
    MajorType(MajorTypeToken),
    /// type defined as an alias or associated entity
    AliasOrAssociateType(TypeToken),
    Trait(TraitToken),
    Const(ConstToken),
}

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FnToken {
    token_idx: TokenIdx,
}

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GnToken {
    token_idx: TokenIdx,
}

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VnToken {
    token_idx: TokenIdx,
}

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PnToken {
    token_idx: TokenIdx,
}

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QnToken {
    token_idx: TokenIdx,
}

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TnToken {
    token_idx: TokenIdx,
}

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConstToken {
    token_idx: TokenIdx,
}

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for ConstToken
where
    Context: TokenStreamParser<'a>,
{
    type Error = TokenDataError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenDataResult<Option<Self>> {
        let token_stream: &mut TokenStream<'a> = &mut ctx.borrow_mut();
        let Some((token_idx, token)) = token_stream.next_indexed() else {
            return Ok(None);
        };
        match token {
            TokenData::Keyword(Keyword::Const) => Ok(Some(ConstToken { token_idx })),
            TokenData::Error(error) => Err(error)?,
            _ => Ok(None),
        }
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StaticToken {
    token_idx: TokenIdx,
}

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormalEntityToken {
    Def(DefToken),
    Lemma(LemmaToken),
    Proposition(PropositionToken),
    Theorem(TheoremToken),
}

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DefToken {
    token_idx: TokenIdx,
}

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LemmaToken {
    token_idx: TokenIdx,
}

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PropositionToken {
    token_idx: TokenIdx,
}

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TheoremToken {
    token_idx: TokenIdx,
}

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FunctionToken {
    token_idx: TokenIdx,
}

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MajorTypeToken {
    keyword: TypeEntityKeyword,
    token_idx: TokenIdx,
}

impl MajorTypeToken {
    pub fn type_kind(self) -> TypeKind {
        match self.keyword {
            TypeEntityKeyword::Extern => TypeKind::Extern,
            TypeEntityKeyword::Struct => TypeKind::Struct,
            TypeEntityKeyword::Enum => TypeKind::Enum,
            TypeEntityKeyword::Record => TypeKind::Record,
            TypeEntityKeyword::Structure => TypeKind::Structure,
            TypeEntityKeyword::Inductive => TypeKind::Inductive,
        }
    }

    pub fn keyword(self) -> TypeEntityKeyword {
        self.keyword
    }

    pub fn token_idx(self) -> TokenIdx {
        self.token_idx
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TypeToken {
    token_idx: TokenIdx,
}

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TraitToken {
    token_idx: TokenIdx,
}

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemoToken {
    token_idx: TokenIdx,
}

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValToken {
    token_idx: TokenIdx,
}

impl ValToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.token_idx
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModToken {
    token_idx: TokenIdx,
}

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for EntityKindKeywordGroup
where
    Context: TokenStreamParser<'a>,
{
    type Error = TokenDataError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenDataResult<Option<Self>> {
        let token_stream: &mut TokenStream<'a> = &mut ctx.borrow_mut();
        let Some((token_idx, token)) = token_stream.next_indexed() else {
            return Ok(None);
        };
        let kw = match token {
            TokenData::Keyword(kw) => kw,
            TokenData::Error(error) => Err(error)?,
            _ => return Ok(None),
        };
        match kw {
            Keyword::Fugitive(kw) => match kw {
                FugitiveKeyword::Val => {
                    Ok(Some(EntityKindKeywordGroup::Ki(ValToken { token_idx })))
                }
                FugitiveKeyword::Fn => Ok(Some(EntityKindKeywordGroup::Fn(FnToken { token_idx }))),
                FugitiveKeyword::Vn => Ok(Some(EntityKindKeywordGroup::Vn(VnToken { token_idx }))),
                FugitiveKeyword::Gn => Ok(Some(EntityKindKeywordGroup::Gn(GnToken { token_idx }))),
                FugitiveKeyword::Pn => Ok(Some(EntityKindKeywordGroup::Pn(PnToken { token_idx }))),
                FugitiveKeyword::Qn => Ok(Some(EntityKindKeywordGroup::Qn(QnToken { token_idx }))),
                FugitiveKeyword::Tn => Ok(Some(EntityKindKeywordGroup::Tn(TnToken { token_idx }))),
                FugitiveKeyword::Def => Ok(Some(EntityKindKeywordGroup::FormalEntity(
                    FormalEntityToken::Def(DefToken { token_idx }),
                ))),
                FugitiveKeyword::Theorem => Ok(Some(EntityKindKeywordGroup::FormalEntity(
                    FormalEntityToken::Theorem(TheoremToken { token_idx }),
                ))),
                FugitiveKeyword::Lemma => Ok(Some(EntityKindKeywordGroup::FormalEntity(
                    FormalEntityToken::Lemma(LemmaToken { token_idx }),
                ))),
                FugitiveKeyword::Proposition => Ok(Some(EntityKindKeywordGroup::FormalEntity(
                    FormalEntityToken::Proposition(PropositionToken { token_idx }),
                ))),
                FugitiveKeyword::Type => Ok(Some(EntityKindKeywordGroup::AliasOrAssociateType(
                    TypeToken { token_idx },
                ))),
                FugitiveKeyword::Memo => {
                    Ok(Some(EntityKindKeywordGroup::Memo(MemoToken { token_idx })))
                }
            },
            Keyword::TypeEntity(keyword) => {
                Ok(Some(EntityKindKeywordGroup::MajorType(MajorTypeToken {
                    keyword,
                    token_idx,
                })))
            }
            Keyword::Stmt(_) => Ok(None),
            Keyword::Mod => Ok(Some(EntityKindKeywordGroup::Submodule(ModToken {
                token_idx,
            }))),
            Keyword::Trait => Ok(Some(EntityKindKeywordGroup::Trait(TraitToken {
                token_idx,
            }))),
            Keyword::Const => Ok(Some(EntityKindKeywordGroup::Const(ConstToken {
                token_idx,
            }))),
            Keyword::Static => match token_stream.peek() {
                Some(TokenData::Keyword(Keyword::Fugitive(FugitiveKeyword::Fn))) => {
                    token_stream.next();
                    Ok(Some(EntityKindKeywordGroup::StaticFn(
                        StaticToken { token_idx },
                        FnToken {
                            token_idx: token_idx + 1,
                        },
                    )))
                }
                _ => Ok(None),
            },
            _ => Ok(None),
        }
    }
}
