```rust
TokenSheetData {
    tokens: [
        TokenData::Keyword(
            Keyword::Stmt(
                Require,
            ),
        ),
        TokenData::Ident(
            `Task`,
        ),
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::Colon,
            ),
        ),
        TokenData::Ident(
            `ml_task`,
        ),
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::Binary(
                    SynBinaryOpr::ScopeResolution,
                ),
            ),
        ),
        TokenData::Ident(
            `IsMlTask`,
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
            ],
        },
        nested_sequences: [],
    },
}
```