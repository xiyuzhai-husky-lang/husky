#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum CharLiteral {
    /// '( ~[' \ \n \r \t] | QUOTE_ESCAPE | ASCII_ESCAPE | UNICODE_ESCAPE )'
    Basic(char),
    /// '\''
    SingeQuoteEscape,
    /// '\''
    DoubleQuoteEscape,
    /// TODO
    AsciiEscape,
    /// TODO
    UnicodeEscape,
}
