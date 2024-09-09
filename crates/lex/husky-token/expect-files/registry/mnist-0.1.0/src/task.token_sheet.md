```rust
TokenSheetData {
    tokens: [
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::Pound,
            ),
        ),
        TokenData::Ident(
            `task`,
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
            `MnistTask`,
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
            `MnistTask`,
        ),
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::Colon,
            ),
        ),
        TokenData::Keyword(
            Keyword::Pub,
        ),
        TokenData::Keyword(
            Keyword::Assoc,
        ),
        TokenData::Keyword(
            Keyword::Form(
                Fn,
            ),
        ),
        TokenData::Ident(
            `new`,
        ),
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::LeftDelimiter(
                    Delimiter::Par,
                ),
            ),
        ),
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::RightDelimiter(
                    Delimiter::Par,
                ),
            ),
        ),
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::Binary(
                    SynBinaryOpr::CurryType,
                ),
            ),
        ),
        TokenData::Keyword(
            Keyword::Pronoun(
                SelfType,
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
                            3,
                        ),
                    ),
                    indent: 0,
                },
                TokenVerseData {
                    start: TokenVerseStart(
                        TokenIdx(
                            7,
                        ),
                    ),
                    indent: 0,
                },
                TokenVerseData {
                    start: TokenVerseStart(
                        TokenIdx(
                            10,
                        ),
                    ),
                    indent: 4,
                },
            ],
        },
        nested_sequences: [],
    },
}
```