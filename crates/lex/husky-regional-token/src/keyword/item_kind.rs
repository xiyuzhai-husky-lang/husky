use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FormFnRegionalToken {
    regional_token_idx: RegionalTokenIdx,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConstRegionalToken {
    regional_token_idx: RegionalTokenIdx,
}

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for ConstRegionalToken
where
    Context: RegionalTokenStreamParser<'a>,
{
    type Error = TokenDataError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenDataResult<Option<Self>> {
        let token_stream: &mut RegionalTokenStream<'a> = &mut ctx.borrow_mut();
        let Some((regional_token_idx, token)) = token_stream.next_indexed() else {
            return Ok(None);
        };
        match token {
            Token::Keyword(Keyword::Const) => Ok(Some(ConstRegionalToken { regional_token_idx })),
            Token::Error(error) => Err(error)?,
            _ => Ok(None),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StaticToken {
    regional_token_idx: RegionalTokenIdx,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeneralDefToken {
    Def(DefToken),
    Lemma(LemmaToken),
    Theorem(TheoremToken),
    Function(FunctionToken),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DefToken {
    regional_token_idx: RegionalTokenIdx,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LemmaToken {
    regional_token_idx: RegionalTokenIdx,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TheoremToken {
    regional_token_idx: RegionalTokenIdx,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FunctionRegionalToken {
    regional_token_idx: RegionalTokenIdx,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TypeEntityRegionalToken {
    keyword: TypeEntityKeyword,
    regional_token_idx: RegionalTokenIdx,
}

impl TypeEntityRegionalToken {
    pub fn type_kind(self) -> TypeKind {
        // MOM
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

    pub fn regional_token_idx(self) -> RegionalTokenIdx {
        self.regional_token_idx
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegionalTypeToken {
    regional_token_idx: RegionalTokenIdx,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TraitRegionalToken {
    regional_token_idx: RegionalTokenIdx,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemoRegionalToken {
    regional_token_idx: RegionalTokenIdx,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValRegionalToken {
    regional_token_idx: RegionalTokenIdx,
}

impl ValRegionalToken {
    pub fn regional_token_idx(&self) -> RegionalTokenIdx {
        self.regional_token_idx
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModRegionalToken {
    regional_token_idx: RegionalTokenIdx,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegionalGnRegionalToken {
    regional_token_idx: RegionalTokenIdx,
}
