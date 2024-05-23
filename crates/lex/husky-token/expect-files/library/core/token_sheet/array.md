```rust
TokenSheetData {
    tokens: [
        TokenData::Keyword(
            Keyword::Use,
        ),
        TokenData::Keyword(
            Keyword::Pronoun(
                Crate,
            ),
        ),
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::Binary(
                    SynBinaryOpr::ScopeResolution,
                ),
            ),
        ),
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::Star,
            ),
        ),
        TokenData::Keyword(
            Keyword::Pub,
        ),
        TokenData::Keyword(
            Keyword::TypeEntity(
                Extern,
            ),
        ),
        TokenData::Ident(
            `Array`,
        ),
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::LaOrLt,
            ),
        ),
        TokenData::Keyword(
            Keyword::Form(
                Termic,
            ),
        ),
        TokenData::Ident(
            `L`,
        ),
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::Colon,
            ),
        ),
        TokenData::Ident(
            `usize`,
        ),
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::Comma,
            ),
        ),
        TokenData::Keyword(
            Keyword::Modifier(
                Covariant,
            ),
        ),
        TokenData::Ident(
            `E`,
        ),
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::RaOrGt,
            ),
        ),
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::Semicolon,
            ),
        ),
    ],
    token_verses: TokenVerses {
        main_sequence: MainTokenVerseSequence {
            verses_data: [
                TokenVerseData {
                    start: TokenVerseStart(
                        TokenIdx(
                            1,
                        ),
                    ),
                    indent: 0,
                },
                TokenVerseData {
                    start: TokenVerseStart(
                        TokenIdx(
                            5,
                        ),
                    ),
                    indent: 0,
                },
            ],
        },
        nested_sequences: [],
    },
}
```