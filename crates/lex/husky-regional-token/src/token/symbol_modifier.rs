use crate::*;
use husky_term_prelude::{Contract, VariableModifier};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EphemSymbolModifierRegionalTokens {
    Owned(DoubleExclamationRegionalToken),
    Mut(MutRegionalToken),
    Ref(RefRegionalToken),
    RefMut(RefRegionalToken, MutRegionalToken),
    Ambersand(AmbersandRegionalToken, Option<LifetimeLabelRegionalToken>),
    AmbersandMut(
        AmbersandRegionalToken,
        Option<LifetimeLabelRegionalToken>,
        MutRegionalToken,
    ),
    Le(LeRegionalToken),
    Tilde(TildeRegionalToken),
    At(AtRegionalToken, Option<PlaceLabelRegionalToken>),
}

impl Into<VariableModifier> for EphemSymbolModifierRegionalTokens {
    #[inline(always)]
    fn into(self) -> VariableModifier {
        match self {
            EphemSymbolModifierRegionalTokens::Owned(_) => VariableModifier::Owned,
            EphemSymbolModifierRegionalTokens::Mut(_) => VariableModifier::Mut,
            EphemSymbolModifierRegionalTokens::Ref(_) => VariableModifier::Ref,
            EphemSymbolModifierRegionalTokens::RefMut(..) => VariableModifier::RefMut,
            EphemSymbolModifierRegionalTokens::Ambersand(_, lifetime_token) => {
                VariableModifier::Ambersand(lifetime_token.map(|t| t.label()))
            }
            EphemSymbolModifierRegionalTokens::AmbersandMut(_, lifetime_token, _) => {
                VariableModifier::AmbersandMut(lifetime_token.map(|t| t.label()))
            }
            EphemSymbolModifierRegionalTokens::Le(..) => VariableModifier::Le,
            EphemSymbolModifierRegionalTokens::Tilde(..) => VariableModifier::Tilde,
            EphemSymbolModifierRegionalTokens::At(_, _) => VariableModifier::At,
        }
    }
}

// we try to keep as similar to `Rust` as possible
impl Into<Contract> for EphemSymbolModifierRegionalTokens {
    #[inline(always)]
    fn into(self) -> Contract {
        match self {
            EphemSymbolModifierRegionalTokens::Mut(_)
            | EphemSymbolModifierRegionalTokens::Owned(_) => Contract::Move,
            EphemSymbolModifierRegionalTokens::Ref(_) => Contract::Borrow,
            EphemSymbolModifierRegionalTokens::RefMut(..) => Contract::BorrowMut,
            EphemSymbolModifierRegionalTokens::Ambersand(_, _) => Contract::Borrow,
            EphemSymbolModifierRegionalTokens::AmbersandMut(_, _, _) => Contract::BorrowMut,
            EphemSymbolModifierRegionalTokens::Le(_) => todo!(),
            EphemSymbolModifierRegionalTokens::Tilde(_) => Contract::Leash,
            EphemSymbolModifierRegionalTokens::At(_, _) => Contract::At,
        }
    }
}

// todo: change this to TryParse
impl<'a, SP> parsec::TryParseOptionFromStream<SP> for EphemSymbolModifierRegionalTokens
where
    SP: RegionalTokenStreamParser<'a>,
{
    type Error = TokenDataError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        sp: &mut SP,
    ) -> TokenDataResult<Option<Self>> {
        let token_stream: &mut RegionalTokenStream<'a> = &mut sp.borrow_mut();
        let Some((regional_token_idx, token)) = token_stream.next_indexed() else {
            return Ok(None);
        };
        match token {
            TokenData::Keyword(Keyword::Modifier(kw)) => match kw {
                ModifierKeyword::Mut => Ok(Some(EphemSymbolModifierRegionalTokens::Mut(
                    MutRegionalToken { regional_token_idx },
                ))),
                ModifierKeyword::Covariant
                | ModifierKeyword::Contravariant
                | ModifierKeyword::Invariant => Ok(None),
                ModifierKeyword::Ref => {
                    let ref_regional_token = RefRegionalToken { regional_token_idx };
                    if let Some(mut_regional_token) =
                        token_stream.try_parse_option::<MutRegionalToken>()?
                    {
                        Ok(Some(EphemSymbolModifierRegionalTokens::RefMut(
                            ref_regional_token,
                            mut_regional_token,
                        )))
                    } else {
                        Ok(Some(EphemSymbolModifierRegionalTokens::Ref(
                            ref_regional_token,
                        )))
                    }
                }
                ModifierKeyword::Le => todo!(),
            },
            TokenData::Punctuation(Punctuation::DOUBLE_EXCLAMATION) => {
                Ok(Some(EphemSymbolModifierRegionalTokens::Owned(
                    DoubleExclamationRegionalToken(regional_token_idx),
                )))
            }
            TokenData::Punctuation(Punctuation::AMBERSAND) => {
                let lifetime_token =
                    token_stream.try_parse_option::<LifetimeLabelRegionalToken>()?;
                if let Some(mut_token) = token_stream.try_parse_option::<MutRegionalToken>()? {
                    Ok(Some(EphemSymbolModifierRegionalTokens::AmbersandMut(
                        AmbersandRegionalToken(regional_token_idx),
                        lifetime_token,
                        mut_token,
                    )))
                } else {
                    Ok(Some(EphemSymbolModifierRegionalTokens::Ambersand(
                        AmbersandRegionalToken(regional_token_idx),
                        lifetime_token,
                    )))
                }
            }
            TokenData::Punctuation(Punctuation::TILDE) => Ok(Some(
                EphemSymbolModifierRegionalTokens::Tilde(TildeRegionalToken(regional_token_idx)),
            )),
            TokenData::Punctuation(Punctuation::AT) => {
                Ok(Some(EphemSymbolModifierRegionalTokens::At(
                    AtRegionalToken(regional_token_idx),
                    token_stream.try_parse_option()?,
                )))
            }
            TokenData::Error(error) => Err(error)?,
            _ => Ok(None),
        }
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct MutRegionalToken {
    regional_token_idx: RegionalTokenIdx,
}

impl MutRegionalToken {
    pub fn regional_token_idx(&self) -> RegionalTokenIdx {
        self.regional_token_idx
    }
}

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for MutRegionalToken
where
    Context: RegionalTokenStreamParser<'a>,
{
    type Error = TokenDataError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenDataResult<Option<Self>> {
        if let Some((regional_token_idx, token)) = ctx.borrow_mut().next_indexed() {
            match token {
                TokenData::Keyword(Keyword::Modifier(ModifierKeyword::Mut)) => {
                    Ok(Some(MutRegionalToken { regional_token_idx }))
                }
                TokenData::Error(error) => Err(error),
                TokenData::Label(_)
                | TokenData::Punctuation(_)
                | TokenData::Ident(_)
                | TokenData::WordOpr(_)
                | TokenData::Literal(_)
                | TokenData::Keyword(_) => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}

/// `ref`
#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct RefRegionalToken {
    regional_token_idx: RegionalTokenIdx,
}

impl RefRegionalToken {
    pub fn regional_token_idx(&self) -> RegionalTokenIdx {
        self.regional_token_idx
    }
}

/// `le`
#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct LeRegionalToken {
    regional_token_idx: RegionalTokenIdx,
}

impl LeRegionalToken {
    pub fn regional_token_idx(&self) -> RegionalTokenIdx {
        self.regional_token_idx
    }
}
