use super::*;

impl<'a> ScopeLRParser<'a> {
    pub(crate) fn special(&mut self, target: Special) -> Option<()> {
        self.kind(target.into())
    }

    pub(crate) fn usize_literal(&mut self) -> Option<usize> {
        if let Some(Token {
            kind: TokenKind::I32Literal(i),
            ..
        }) = self.stream.next()
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

    pub(crate) fn custom_ident(&mut self) -> Option<CustomIdentifier> {
        if let Some(Token {
            kind: TokenKind::Identifier(Identifier::Custom(ident)),
            ..
        }) = self.stream.next()
        {
            Some(*ident)
        } else {
            None
        }
    }

    fn kind(&mut self, target: TokenKind) -> Option<()> {
        if let Some(Token { kind, .. }) = self.stream.next() {
            if *kind == target {
                return Some(());
            }
        }
        None
    }
}
