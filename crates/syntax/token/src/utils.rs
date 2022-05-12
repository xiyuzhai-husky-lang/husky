#[macro_export]
macro_rules! identify {
    ($this: expr, $token:expr, $semantic_token_kind: expr) => {{
        match $token.kind {
            TokenKind::Identifier(Identifier::Custom(ident)) => {
                $this.push_abs_semantic_token(token::AbsSemanticToken::new(
                    $semantic_token_kind,
                    $token.range,
                ));
                text::RangedCustomIdentifier {
                    ident,
                    range: $token.range,
                }
            }
            _ => err!("expect `<custom_identifier>`", $token.range)?,
        }
    }};
}

#[macro_export]
macro_rules! must_be {
    ($cond:expr, $msg: expr, $range:expr) => {
        if !$cond {
            err!($msg, $range)?
        }
    };
}

#[macro_export]
macro_rules! expect_kind {
    ($token:expr, $kind:expr) => {
        must_be!(
            $token.kind == TokenKind::Special($kind),
            format!("expect `{}`", $kind.code()),
            $token.range
        );
    };
}

#[macro_export]
macro_rules! expect_block_head {
    ($tokens:expr) => {
        expect_kind!($tokens.last().unwrap(), Special::Colon)
    };
}

#[macro_export]
macro_rules! expect_at_least {
    ($tokens:expr, $kw_range:expr, $lower_bound:expr) => {
        must_be!(
            $tokens.len() >= $lower_bound,
            format!(
                "expect at least {} tokens after keyword, but got {} tokens instead",
                $lower_bound,
                $tokens.len()
            ),
            $kw_range
        );
    };
}

#[macro_export]
macro_rules! expect_len {
    ($tokens:expr,  $len:expr) => {
        must_be!(
            $tokens.len() == $len,
            format!(
                "expect {} tokens after keyword, but got {} tokens instead",
                $len,
                $tokens.len()
            ),
            $tokens.text_range()
        );
    };
}

#[macro_export]
macro_rules! trim_colon {
    ($tokens:expr; keyword, colon) => {{
        expect_kind!($tokens.last().unwrap(), Special::Colon);
        &$tokens[1..($tokens.len() - 1)]
    }};

    ($tokens:expr, $kw_range:expr) => {{
        expect_at_least!($tokens, $kw_range, 1);
        expect_kind!($tokens.last().unwrap(), Special::Colon);
        &$tokens[0..($tokens.len() - 1)]
    }};
}

#[macro_export]
macro_rules! expect_head {
    ($tokens:expr) => {{
        expect_kind!($tokens.last().unwrap(), Special::Colon);
    }};
}
