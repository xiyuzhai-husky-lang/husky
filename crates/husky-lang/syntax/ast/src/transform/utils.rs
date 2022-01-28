macro_rules! identify {
    ($token:expr) => {{
        match $token.kind {
            TokenKind::Identifier(Identifier::Custom(ident)) => ident,
            _ => ast_err!($token.range, "expect `<custom_identifier>`")?,
        }
    }};
}
pub(super) use identify;

macro_rules! expect {
    ($cond:expr, $range:expr, $msg:expr) => {
        if !$cond {
            ast_err!($range, $msg)?
        }
    };
}
pub(super) use expect;

macro_rules! expect_kind {
    ($token:expr, $kind:expr) => {
        expect!(
            $token.kind == TokenKind::Special($kind),
            $token.range,
            format!("expect `{}`", $kind.code())
        );
    };
}
pub(super) use expect_kind;

macro_rules! expect_at_least {
    ($tokens:expr, $kw_range:expr, $lower_bound:expr) => {
        expect!(
            $tokens.len() >= $lower_bound,
            $kw_range,
            format!(
                "expect at least {} tokens after keyword, but got {} tokens instead",
                $lower_bound,
                $tokens.len()
            )
        );
    };
}
pub(super) use expect_at_least;

macro_rules! expect_len {
    ($tokens:expr,  $len:expr) => {
        expect!(
            $tokens.len() == $len,
            text::group_text_range($tokens),
            format!(
                "expect {} tokens after keyword, but got {} tokens instead",
                $len,
                $tokens.len()
            )
        );
    };
}
pub(super) use expect_len;

macro_rules! trim {
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
pub(super) use trim;

macro_rules! expect_head {
    ($tokens:expr) => {{
        expect_kind!($tokens.last().unwrap(), Special::Colon);
    }};
}
pub(super) use expect_head;
