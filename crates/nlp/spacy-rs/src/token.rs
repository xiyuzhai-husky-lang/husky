pub mod span;

use crate::*;
use pyo3::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromPyObject)]
pub struct Token {
    /// The original text of the token
    pub text: String,
    /// The index of the token within the parent document
    pub i: usize,
    /// Base form of the token, with no inflectional suffixes
    pub lemma_: String,
    /// Normalized form of the token
    pub norm_: String,
    /// The characters at the start of the token
    pub prefix_: String,
    /// The characters at the end of the token
    pub suffix_: String,
    /// Does the token consist of ASCII characters?
    /// Equivalent to: all(ord(c) < 128 for c in token.text)
    pub is_ascii: bool,
    /// Does the token consist of digits?
    /// Equivalent to: token.text.isdigit()
    pub is_digit: bool,
    /// Is the token in lowercase?
    pub is_lower: bool,
    /// Is the token in uppercase?
    pub is_upper: bool,
    /// Is the token in titlecase?
    pub is_title: bool,
    /// Is the token punctuation?
    pub is_punct: bool,
    /// Is the token a left punctuation mark? e.g. "("
    pub is_left_punct: bool,
    /// Is the token a right punctuation mark? e.g. ")"
    pub is_right_punct: bool,
    /// Does the token start a sentence?
    /// None if unknown. Defaults to True for the first token in the Doc
    pub is_sent_start: Option<bool>,
    /// Does the token end a sentence?
    /// None if unknown
    pub is_sent_end: Option<bool>,
    /// Does the token consist of whitespace characters?
    /// Equivalent to: token.text.isspace()
    pub is_space: bool,
    /// Is the token a bracket?
    pub is_bracket: bool,
    /// Is the token a quotation mark?
    pub is_quote: bool,
    /// Is the token a currency symbol?
    pub is_currency: bool,
    /// Does the token resemble a URL?
    pub like_url: bool,
    /// Does the token represent a number?
    /// Examples: "10.9", "10", "ten", etc.
    pub like_num: bool,
    /// Does the token resemble an email address?
    pub like_email: bool,
    /// Is the token out-of-vocabulary (i.e. does it not have a word vector)?
    pub is_oov: bool,
    /// Is the token part of a "stop list"?
    pub is_stop: bool,
    /// Coarse-grained part-of-speech from the Universal POS tag set
    pub pos: i32,
    /// String representation of the coarse-grained part-of-speech
    pub pos_: String,
}
