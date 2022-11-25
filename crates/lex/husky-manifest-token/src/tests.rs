use husky_word::{WordDb, WordJar};
use salsa::Database;

use super::{Error, TokenVariant, Tokenizer};
use std::{borrow::Cow, sync::Arc};

#[salsa::db(WordJar)]
#[derive(Default)]
pub struct MimicDB {
    storage: salsa::Storage<Self>,
}

impl Database for MimicDB {}

fn err(input: &str, err: Error) {
    let db = MimicDB::default();
    let mut t = Tokenizer::new(&db, input);
    let token = t.next().unwrap_err();
    assert_eq!(token, err);
    assert!(t.next().unwrap().is_none());
}

#[test]
fn literal_strings() {
    fn t(db: &dyn WordDb, input: &str, val: &str, multiline: bool) {
        let mut t = Tokenizer::new(db, input);
        let token = t.next().unwrap().unwrap();
        assert_eq!(
            token.variant,
            TokenVariant::StringLiteral {
                val: Arc::new(val.to_owned()),
                multiline,
            }
        );
        assert!(t.next().unwrap().is_none());
    }

    let db = MimicDB::default();
    t(&db, "''", "", false);
    t(&db, "''''''", "", true);
    t(&db, "'''\n'''", "", true);
    t(&db, "'a'", "a", false);
    t(&db, "'\"a'", "\"a", false);
    t(&db, "''''a'''", "'a", true);
    t(&db, "'''\n'a\n'''", "'a\n", true);
    t(&db, "'''a\n'a\r\n'''", "a\n'a\n", true);
}

#[test]
fn basic_strings() {
    fn t(db: &dyn WordDb, input: &str, val: &str, multiline: bool) {
        let mut t = Tokenizer::new(db, input);
        let token = t.next().unwrap().unwrap();
        assert_eq!(
            token.variant,
            TokenVariant::StringLiteral {
                val: Arc::new(val.to_owned()),
                multiline,
            }
        );
        assert!(t.next().unwrap().is_none());
    }

    let db = MimicDB::default();
    t(&db, r#""""#, "", false);
    t(&db, r#""""""""#, "", true);
    t(&db, r#""a""#, "a", false);
    t(&db, r#""""a""""#, "a", true);
    t(&db, r#""\t""#, "\t", false);
    t(&db, r#""\u0000""#, "\0", false);
    t(&db, r#""\U00000000""#, "\0", false);
    t(&db, r#""\U000A0000""#, "\u{A0000}", false);
    t(&db, r#""\\t""#, "\\t", false);
    t(&db, "\"\t\"", "\t", false);
    t(&db, "\"\"\"\n\t\"\"\"", "\t", true);
    t(&db, "\"\"\"\\\n\"\"\"", "", true);
    t(
        &db,
        "\"\"\"\\\n     \t   \t  \\\r\n  \t \n  \t \r\n\"\"\"",
        "",
        true,
    );
    t(&db, r#""\r""#, "\r", false);
    t(&db, r#""\n""#, "\n", false);
    t(&db, r#""\b""#, "\u{8}", false);
    t(&db, r#""a\fa""#, "a\u{c}a", false);
    t(&db, r#""\"a""#, "\"a", false);
    t(&db, "\"\"\"\na\"\"\"", "a", true);
    t(&db, "\"\"\"\n\"\"\"", "", true);
    t(&db, r#""""a\"""b""""#, "a\"\"\"b", true);
    err(r#""\a"#, Error::InvalidEscape(2, 'a'));
    err("\"\\\n", Error::InvalidEscape(2, '\n'));
    err("\"\\\r\n", Error::InvalidEscape(2, '\n'));
    err("\"\\", Error::UnterminatedString(0));
    err("\"\u{0}", Error::InvalidCharInString(1, '\u{0}'));
    err(r#""\U00""#, Error::InvalidHexEscape(5, '"'));
    err(r#""\U00"#, Error::UnterminatedString(0));
    err(r#""\uD800"#, Error::InvalidEscapeValue(2, 0xd800));
    err(r#""\UFFFFFFFF"#, Error::InvalidEscapeValue(2, 0xffff_ffff));
}

#[test]
fn keylike() {
    fn t(db: &dyn WordDb, input: &str) {
        let mut t = Tokenizer::new(db, input);
        let token = t.next().unwrap().unwrap();
        assert_eq!(
            token.variant,
            TokenVariant::Keylike(db.it_word_borrowed(input))
        );
        assert!(t.next().unwrap().is_none());
    }

    let db = MimicDB::default();
    t(&db, "foo");
    t(&db, "0bar");
    t(&db, "bar0");
    t(&db, "1234");
    t(&db, "a-b");
    t(&db, "a_B");
    t(&db, "-_-");
    t(&db, "___");
}

#[test]
fn all() {
    fn t(db: &dyn WordDb, input: &str, expected: &[((usize, usize), TokenVariant, &str)]) {
        let mut tokens = Tokenizer::new(db, input);
        let mut actual: Vec<((usize, usize), TokenVariant, &str)> = Vec::new();
        while let Some(token) = tokens.next().unwrap() {
            actual.push((
                token.span.into(),
                token.variant,
                &input[token.span.start..token.span.end],
            ));
        }
        for (a, b) in actual.iter().zip(expected) {
            assert_eq!(a, b);
        }
        assert_eq!(actual.len(), expected.len());
    }

    let db = MimicDB::default();
    t(
        &db,
        " a ",
        &[
            ((0, 1), TokenVariant::Whitespace, " "),
            ((1, 2), TokenVariant::Keylike(db.it_word_borrowed("a")), "a"),
            ((2, 3), TokenVariant::Whitespace, " "),
        ],
    );

    t(
        &db,
        " a\t [[]] \t [] {} , . =\n# foo \r\n#foo \n ",
        &[
            ((0, 1), TokenVariant::Whitespace, " "),
            ((1, 2), TokenVariant::Keylike(db.it_word_borrowed("a")), "a"),
            ((2, 4), TokenVariant::Whitespace, "\t "),
            ((4, 5), TokenVariant::LeftBracket, "["),
            ((5, 6), TokenVariant::LeftBracket, "["),
            ((6, 7), TokenVariant::RightBracket, "]"),
            ((7, 8), TokenVariant::RightBracket, "]"),
            ((8, 11), TokenVariant::Whitespace, " \t "),
            ((11, 12), TokenVariant::LeftBracket, "["),
            ((12, 13), TokenVariant::RightBracket, "]"),
            ((13, 14), TokenVariant::Whitespace, " "),
            ((14, 15), TokenVariant::LeftBrace, "{"),
            ((15, 16), TokenVariant::RightBrace, "}"),
            ((16, 17), TokenVariant::Whitespace, " "),
            ((17, 18), TokenVariant::Comma, ","),
            ((18, 19), TokenVariant::Whitespace, " "),
            ((19, 20), TokenVariant::Period, "."),
            ((20, 21), TokenVariant::Whitespace, " "),
            ((21, 22), TokenVariant::Equals, "="),
            ((22, 23), TokenVariant::Newline, "\n"),
            ((23, 29), TokenVariant::Comment, "# foo "),
            ((29, 31), TokenVariant::Newline, "\r\n"),
            ((31, 36), TokenVariant::Comment, "#foo "),
            ((36, 37), TokenVariant::Newline, "\n"),
            ((37, 38), TokenVariant::Whitespace, " "),
        ],
    );
}

#[test]
fn bare_cr_bad() {
    err("\r", Error::Unexpected(0, '\r'));
    err("'\n", Error::NewlineInString(1));
    err("'\u{0}", Error::InvalidCharInString(1, '\u{0}'));
    err("'", Error::UnterminatedString(0));
    err("\u{0}", Error::Unexpected(0, '\u{0}'));
}

#[test]
fn bad_comment() {
    let db = MimicDB::default();
    let mut t = Tokenizer::new(&db, "#\u{0}");
    t.next().unwrap().unwrap();
    assert_eq!(t.next(), Err(Error::Unexpected(1, '\u{0}')));
    assert!(t.next().unwrap().is_none());
}
