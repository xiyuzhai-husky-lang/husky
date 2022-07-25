use husky_primitive_literal_syntax::PrimitiveLiteralData;
use husky_text::RangedCustomIdentifier;
use husky_word::{Keyword, LiasonKeyword, Paradigm};

use super::*;

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
                PrimitiveLiteralData::Integer(_) => todo!(),
                PrimitiveLiteralData::I64(_) => todo!(),
                PrimitiveLiteralData::Float(_) => todo!(),
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
