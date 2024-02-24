/// simplied for Latex
pub enum MathToken {
    Command(MathCommand),
    Delimiter(MathDelimiter),
    Other(char),
}

pub enum MathCommand {
    Frac,
    Abs,
    Norm,
    Ang,
    Perp,
    Int,
}

pub enum MathDelimiter {
    /// '{', '}'
    Curl,
    /// '[', ']'
    Box,
    /// '(', ')'
    Par,
}

pub enum Script {}
