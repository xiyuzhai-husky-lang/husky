use husky_text::RangedCustomIdentifier;
use vm::PrimitiveValueData;
use word::{Keyword, LiasonKeyword, Paradigm};

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
            kind: HuskyTokenKind::PrimitiveLiteral(PrimitiveValueData::I32(i)),
            ..
        }) = self.token_stream.next()
        {
            if *i < 0 {
                None
            } else {
                Some(*i as usize)
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
