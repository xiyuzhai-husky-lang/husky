use text::RangedCustomIdentifier;
use vm::CopyableValue;

use super::*;

impl<'a, 'b> AtomParser<'a, 'b> {
    pub(crate) fn special(&mut self, target: Special) -> Option<()> {
        self.kind(target.into())
    }

    pub(crate) fn usize_literal(&mut self) -> Option<usize> {
        if let Some(Token {
            kind: TokenKind::PrimitiveLiteral(CopyableValue::I32(i)),
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

    pub(crate) fn custom_ident(&mut self) -> Option<RangedCustomIdentifier> {
        if let Some(Token {
            kind: TokenKind::Identifier(Identifier::Custom(ident)),
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

    fn kind(&mut self, target: TokenKind) -> Option<()> {
        if let Some(Token { kind, .. }) = self.token_stream.next() {
            if *kind == target {
                return Some(());
            }
        }
        None
    }
}
