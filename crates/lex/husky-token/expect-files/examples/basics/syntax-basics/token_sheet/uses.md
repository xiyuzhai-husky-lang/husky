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