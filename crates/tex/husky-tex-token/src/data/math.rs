use husky_tex_math_letter::TexMathLetter;

#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TexMathTokenData {
    Command(TexMathCommand),
    Delimiter(TexMathDelimiter),
    Letter(TexMathLetter),
    Other(char),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TexMathCommand {
    Frac,
    Abs,
    Norm,
    Ang,
    Perp,
    Int,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TexMathDelimiter {
    /// '{', '}'
    Curl,
    /// '[', ']'
    Box,
    /// '(', ')'
    Par,
    /// '\{', '\}'
    Set,
}

pub enum Script {}
