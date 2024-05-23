use husky_entity_kind::{ritchie::RitchieItemKind, MajorFormKind};

use super::*;

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityKindKeywordGroup {
    /// todo: remove mod
    Submodule(ModToken),
    Ritchie(RitchieItemKindToken),
    /// `static fn`
    AssocRitchie(AssocToken, RitchieItemKindToken),
    /// `val`
    Val(ValToken),
    /// `memo`
    Memo(MemoToken),
    /// husky will have the capacities of theorem proving
    ConceptualEntity(ConceptualEntityToken),
    /// type defined as a major entity
    MajorType(MajorTypeToken),
    /// type defined as an alias or associated entity
    AliasOrAssociateType(TypeToken),
    Trait(TraitToken),
    Termic(TermicToken),
    Static(StaticToken),
}

#[enum_class::from_variants]
#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RitchieItemKindToken {
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
    /// `tn`
    Bn(BnToken),
    /// `tn`
    Sn(SnToken),
    /// `tn`
    Tn(TnToken),
}

impl RitchieItemKindToken {
    pub fn ritchie_item_kind(self) -> RitchieItemKind {
        match self {
            RitchieItemKindToken::Fn(_) => RitchieItemKind::Fn,
            RitchieItemKindToken::Gn(_) => RitchieItemKind::Gn,
            RitchieItemKindToken::Vn(_) => RitchieItemKind::Vn,
            RitchieItemKindToken::Pn(_) => RitchieItemKind::Pn,
            RitchieItemKindToken::Qn(_) => RitchieItemKind::Qn,
            RitchieItemKindToken::Bn(_) => RitchieItemKind::Bn,
            RitchieItemKindToken::Sn(_) => RitchieItemKind::Sn,
            RitchieItemKindToken::Tn(_) => RitchieItemKind::Tn,
        }
    }
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
pub struct BnToken {
    token_idx: TokenIdx,
}

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SnToken {
    token_idx: TokenIdx,
}

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TnToken {
    token_idx: TokenIdx,
}

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TermicToken {
    token_idx: TokenIdx,
}

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for TermicToken
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
            TokenData::Keyword(Keyword::TERMIC) => Ok(Some(TermicToken { token_idx })),
            TokenData::Error(error) => Err(error)?,
            _ => Ok(None),
        }
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AssocToken {
    token_idx: TokenIdx,
}

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StaticToken {
    token_idx: TokenIdx,
}

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConceptualEntityToken {
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
            Keyword::Form(kw) => match kw {
                FormKeyword::Val => Ok(Some(EntityKindKeywordGroup::Val(ValToken { token_idx }))),
                FormKeyword::Fn => Ok(Some(EntityKindKeywordGroup::Ritchie(
                    FnToken { token_idx }.into(),
                ))),
                FormKeyword::Vn => Ok(Some(EntityKindKeywordGroup::Ritchie(
                    VnToken { token_idx }.into(),
                ))),
                FormKeyword::Gn => Ok(Some(EntityKindKeywordGroup::Ritchie(
                    GnToken { token_idx }.into(),
                ))),
                FormKeyword::Pn => Ok(Some(EntityKindKeywordGroup::Ritchie(
                    PnToken { token_idx }.into(),
                ))),
                FormKeyword::Qn => Ok(Some(EntityKindKeywordGroup::Ritchie(
                    QnToken { token_idx }.into(),
                ))),
                FormKeyword::Bn => Ok(Some(EntityKindKeywordGroup::Ritchie(
                    BnToken { token_idx }.into(),
                ))),
                FormKeyword::Sn => Ok(Some(EntityKindKeywordGroup::Ritchie(
                    SnToken { token_idx }.into(),
                ))),
                FormKeyword::Tn => Ok(Some(EntityKindKeywordGroup::Ritchie(
                    TnToken { token_idx }.into(),
                ))),

                FormKeyword::Def => Ok(Some(EntityKindKeywordGroup::ConceptualEntity(
                    ConceptualEntityToken::Def(DefToken { token_idx }),
                ))),
                FormKeyword::Theorem => Ok(Some(EntityKindKeywordGroup::ConceptualEntity(
                    ConceptualEntityToken::Theorem(TheoremToken { token_idx }),
                ))),
                FormKeyword::Lemma => Ok(Some(EntityKindKeywordGroup::ConceptualEntity(
                    ConceptualEntityToken::Lemma(LemmaToken { token_idx }),
                ))),
                FormKeyword::Proposition => Ok(Some(EntityKindKeywordGroup::ConceptualEntity(
                    ConceptualEntityToken::Proposition(PropositionToken { token_idx }),
                ))),
                FormKeyword::Type => Ok(Some(EntityKindKeywordGroup::AliasOrAssociateType(
                    TypeToken { token_idx },
                ))),
                FormKeyword::Static => Ok(Some(EntityKindKeywordGroup::Static(StaticToken {
                    token_idx,
                }))),
                FormKeyword::Termic => Ok(Some(EntityKindKeywordGroup::Termic(TermicToken {
                    token_idx,
                }))),
                FormKeyword::Memo => {
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
            Keyword::Assoc => match token_stream.peek() {
                Some(TokenData::Keyword(Keyword::Form(form_kw))) => {
                    token_stream.next();
                    let ritchie_item_kind_token = match form_kw {
                        FormKeyword::Type => todo!(),
                        FormKeyword::Fn => FnToken {
                            token_idx: token_idx + 1,
                        }
                        .into(),
                        FormKeyword::Vn => VnToken {
                            token_idx: token_idx + 1,
                        }
                        .into(),
                        FormKeyword::Gn => GnToken {
                            token_idx: token_idx + 1,
                        }
                        .into(),
                        FormKeyword::Pn => PnToken {
                            token_idx: token_idx + 1,
                        }
                        .into(),
                        FormKeyword::Qn => QnToken {
                            token_idx: token_idx + 1,
                        }
                        .into(),
                        FormKeyword::Bn => BnToken {
                            token_idx: token_idx + 1,
                        }
                        .into(),
                        FormKeyword::Sn => SnToken {
                            token_idx: token_idx + 1,
                        }
                        .into(),
                        FormKeyword::Tn => TnToken {
                            token_idx: token_idx + 1,
                        }
                        .into(),

                        FormKeyword::Memo => todo!(),
                        FormKeyword::Static => todo!(),
                        FormKeyword::Termic => todo!(),
                        FormKeyword::Val => todo!(),
                        FormKeyword::Def => todo!(),
                        FormKeyword::Theorem => todo!(),
                        FormKeyword::Lemma => todo!(),
                        FormKeyword::Proposition => todo!(),
                    };
                    Ok(Some(EntityKindKeywordGroup::AssocRitchie(
                        AssocToken { token_idx },
                        ritchie_item_kind_token,
                    )))
                }
                _ => Ok(Some(EntityKindKeywordGroup::Static(StaticToken {
                    token_idx,
                }))),
            },
            _ => Ok(None),
        }
    }
}
