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
            Keyword::Trait,
        ),
        TokenData::Ident(
            `Copy`,
        ),
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::Semicolon,
            ),
        ),
        TokenData::Keyword(
            Keyword::Impl,
        ),
        TokenData::Ident(
            `Copy`,
        ),
        TokenData::Keyword(
            Keyword::Connection(
                For,
            ),
        ),
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::Pound,
            ),
        ),
        TokenData::Ident(
            `derive`,
        ),
        TokenData::Ident(
            `_`,
        ),
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::Semicolon,
            ),
        ),
        TokenData::Keyword(
            Keyword::Pub,
        ),
        TokenData::Keyword(
            Keyword::Trait,
        ),
        TokenData::Ident(
            `Sized`,
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
                TokenVerseData {
                    start: TokenVerseStart(
                        TokenIdx(
                            9,
                        ),
                    ),
                    indent: 0,
                },
                TokenVerseData {
                    start: TokenVerseStart(
                        TokenIdx(
                            16,
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