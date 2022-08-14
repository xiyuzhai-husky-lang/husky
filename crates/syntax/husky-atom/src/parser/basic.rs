use husky_primitive_literal_syntax::PrimitiveLiteralData;
use husky_text::RangedCustomIdentifier;
use husky_word::{Keyword, LiasonKeyword, Paradigm};

use super::*;

pub struct OfHuskyTokenKindPattern(pub HuskyTokenKind);
impl std::fmt::Display for OfHuskyTokenKindPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "token of kind ".fmt(f)?;
        self.0.fmt(f)
    }
}
impl AtomParserPattern for OfHuskyTokenKindPattern {
    type Output = ();

    fn get_parsed(&self, parser: &mut AtomParser) -> AtomResult<Option<Self::Output>> {
        if let Some(HuskyToken { kind, .. }) = parser.token_stream.next() {
            if *kind == self.0 {
                return Ok(Some(()));
            }
        }
        Ok(None)
    }
}

pub struct ParadigmPattern;
impl std::fmt::Display for ParadigmPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "paradigm keyword".fmt(f)
    }
}
impl AtomParserPattern for ParadigmPattern {
    type Output = Paradigm;

    fn get_parsed(&self, parser: &mut AtomParser) -> AtomResult<Option<Self::Output>> {
        Ok(
            if let Some(HuskyToken {
                kind: HuskyTokenKind::Keyword(Keyword::Paradigm(paradigm)),
                ..
            }) = parser.token_stream.next()
            {
                Some(*paradigm)
            } else {
                None
            },
        )
    }
}

pub struct UsizeLiteralPattern;
impl std::fmt::Display for UsizeLiteralPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "usize literal".fmt(f)
    }
}
impl AtomParserPattern for UsizeLiteralPattern {
    type Output = usize;

    fn get_parsed(&self, parser: &mut AtomParser) -> AtomResult<Option<Self::Output>> {
        Ok(
            if let Some(HuskyToken {
                kind: HuskyTokenKind::PrimitiveLiteral(data),
                ..
            }) = parser.token_stream.next()
            {
                match data {
                    PrimitiveLiteralData::Void => todo!(),
                    PrimitiveLiteralData::I32(i) => {
                        if *i < 0 {
                            None
                        } else {
                            Some(*i as usize)
                        }
                    }
                    PrimitiveLiteralData::Integer(i) => {
                        if *i < 0 {
                            None
                        } else {
                            Some(*i as usize)
                        }
                    }
                    PrimitiveLiteralData::I64(_) => todo!(),
                    PrimitiveLiteralData::Float(_) => None,
                    PrimitiveLiteralData::F32(_) => todo!(),
                    PrimitiveLiteralData::F64(_) => todo!(),
                    PrimitiveLiteralData::Bits(_) => todo!(),
                    PrimitiveLiteralData::B32(_) => todo!(),
                    PrimitiveLiteralData::B64(_) => todo!(),
                    PrimitiveLiteralData::Bool(_) => todo!(),
                }
            } else {
                None
            },
        )
    }
}

pub struct BeSpecialTokenPattern(pub SpecialToken);
impl std::fmt::Display for BeSpecialTokenPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl AtomParserPattern for BeSpecialTokenPattern {
    type Output = ();

    fn get_parsed(&self, parser: &mut AtomParser) -> AtomResult<Option<Self::Output>> {
        OfHuskyTokenKindPattern(self.0.into()).get_parsed(parser)
    }
}

#[macro_export]
macro_rules! be_special_token_patt {
    ($s: tt) => {{
        BeSpecialTokenPattern(husky_token::special_token!($s))
    }};
}

impl<'a, 'b> AtomParser<'a, 'b> {
    pub fn special(&mut self, target: SpecialToken) -> Option<()> {
        self.token_kind(target.into())
    }

    pub fn paradigm(&mut self) -> Option<Paradigm> {
        if let Some(HuskyToken {
            kind: HuskyTokenKind::Keyword(Keyword::Paradigm(paradigm)),
            ..
        }) = self.token_stream.next()
        {
            Some(*paradigm)
        } else {
            None
        }
    }

    pub fn usize_literal(&mut self) -> Option<usize> {
        if let Some(HuskyToken {
            kind: HuskyTokenKind::PrimitiveLiteral(data),
            ..
        }) = self.token_stream.next()
        {
            match data {
                PrimitiveLiteralData::Void => todo!(),
                PrimitiveLiteralData::I32(i) => {
                    if *i < 0 {
                        None
                    } else {
                        Some(*i as usize)
                    }
                }
                PrimitiveLiteralData::Integer(i) => {
                    if *i < 0 {
                        None
                    } else {
                        Some(*i as usize)
                    }
                }
                PrimitiveLiteralData::I64(_) => todo!(),
                PrimitiveLiteralData::Float(_) => None,
                PrimitiveLiteralData::F32(_) => todo!(),
                PrimitiveLiteralData::F64(_) => todo!(),
                PrimitiveLiteralData::Bits(_) => todo!(),
                PrimitiveLiteralData::B32(_) => todo!(),
                PrimitiveLiteralData::B64(_) => todo!(),
                PrimitiveLiteralData::Bool(_) => todo!(),
            }
        } else {
            None
        }
    }

    pub fn custom_ident(&mut self) -> Option<RangedCustomIdentifier> {
        if let Some(HuskyToken {
            kind: HuskyTokenKind::Identifier(Identifier::Custom(ident)),
            range,
        }) = self.token_stream.next()
        {
            Some(RangedCustomIdentifier {
                ident: *ident,
                range: *range,
            })
        } else {
            None
        }
    }

    pub fn sema_custom_ident(
        &mut self,
        semantic_token_kind: SemanticTokenKind,
    ) -> Option<RangedCustomIdentifier> {
        if let Some(HuskyToken {
            kind: HuskyTokenKind::Identifier(Identifier::Custom(ident)),
            range,
        }) = self.token_stream.next()
        {
            self.atom_context
                .push_abs_semantic_token(AbsSemanticToken::new(semantic_token_kind, *range));
            Some(RangedCustomIdentifier {
                ident: *ident,
                range: *range,
            })
        } else {
            None
        }
    }

    pub fn token_kind(&mut self, target: HuskyTokenKind) -> Option<()> {
        if let Some(HuskyToken { kind, .. }) = self.token_stream.next() {
            if *kind == target {
                return Some(());
            }
        }
        None
    }

    pub fn liason(&mut self) -> Option<LiasonKeyword> {
        if let Some(HuskyToken { kind, .. }) = self.token_stream.next() {
            match kind {
                HuskyTokenKind::Keyword(Keyword::Liason(liason_keyword)) => {
                    return Some(*liason_keyword)
                }
                _ => (),
            }
        }
        None
    }
}
