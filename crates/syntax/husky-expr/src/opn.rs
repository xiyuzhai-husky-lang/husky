use husky_opn_syntax::{BinaryPunctuation, Bracket, ListOpr, PrefixPunctuation, SuffixPunctuation};
use husky_text::RangedIdentifier;
use husky_token::TokenIdx;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Opn {
    Binary(BinaryPunctuation),
    Prefix(PrefixPunctuation),
    Suffix(SuffixPunctuation),
    CurlBracketed,
    List {
        opr: ListOpr,
        bracket: Bracket,
        bra_token_idx: TokenIdx,
        ket_token_idx: TokenIdx,
    },
    Field(Option<RangedIdentifier>),
    Abstraction,
    Application,
}
