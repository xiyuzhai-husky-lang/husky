#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TexMathOpr {
    /// Addition `+`
    Add,
    /// Subtraction `-`
    Sub,
    /// Multiplication `\cdot`
    Mul,
    /// Division `/`
    Div,
    /// Element of `\in` or `∈`
    In,
    /// Not an element of `\notin` or `∉`
    NotIn,
    /// Subset of `\subset` or `⊂`
    Subset,
    /// Superset of `\supset` or `⊃`
    Superset,
    /// Subset of or equal to `\subseteq` or `⊆`
    SubsetEq,
    /// Superset of or equal to `\supseteq` or `⊇`
    SupersetEq,
    /// For all `\forall` or `∀`
    ForAll,
    /// Exists `\exists` or `∃`
    Exists,
    /// Does not exist `\nexists` or `∄`
    NotExists,
    /// Infinity `\infty` or `∞`
    Infinity,
    /// Equals `=`
    Equals,
    /// Not equals `\neq` or `≠`
    NotEquals,
    /// Less than `<`
    LessThan,
    /// Greater than `>`
    GreaterThan,
    /// Less than or equal to `\leq` or `≤` or `<=`
    LessEq,
    /// Greater than or equal to `\geq` or `≥` `>=`
    GreaterEq,
    /// Plus minus `\pm` or `±`
    PlusMinus,
    /// Times `\times` or `×`
    Times,
}

impl TexMathOpr {
    pub fn to_char(self) -> char {
        match self {
            TexMathOpr::Add => '+',
            TexMathOpr::Sub => '-',
            TexMathOpr::Mul => '·',
            TexMathOpr::Div => '/',
            TexMathOpr::In => '∈',
            TexMathOpr::NotIn => '∉',
            TexMathOpr::Subset => '⊂',
            TexMathOpr::Superset => '⊃',
            TexMathOpr::SubsetEq => '⊆',
            TexMathOpr::SupersetEq => '⊇',
            TexMathOpr::ForAll => '∀',
            TexMathOpr::Exists => '∃',
            TexMathOpr::NotExists => '∄',
            TexMathOpr::Infinity => '∞',
            TexMathOpr::Equals => '=',
            TexMathOpr::NotEquals => '≠',
            TexMathOpr::LessThan => '<',
            TexMathOpr::GreaterThan => '>',
            TexMathOpr::LessEq => '≤',
            TexMathOpr::GreaterEq => '≥',
            TexMathOpr::PlusMinus => '±',
            TexMathOpr::Times => '×',
        }
    }

    pub fn try_from_char(c: char) -> Option<Self> {
        match c {
            '+' => Some(TexMathOpr::Add),
            '-' => Some(TexMathOpr::Sub),
            '·' => Some(TexMathOpr::Mul),
            '/' => Some(TexMathOpr::Div),
            '∈' => Some(TexMathOpr::In),
            '∉' => Some(TexMathOpr::NotIn),
            '⊂' => Some(TexMathOpr::Subset),
            '⊃' => Some(TexMathOpr::Superset),
            '⊆' => Some(TexMathOpr::SubsetEq),
            '⊇' => Some(TexMathOpr::SupersetEq),
            '∀' => Some(TexMathOpr::ForAll),
            '∃' => Some(TexMathOpr::Exists),
            '∄' => Some(TexMathOpr::NotExists),
            '∞' => Some(TexMathOpr::Infinity),
            '=' => Some(TexMathOpr::Equals),
            '≠' => Some(TexMathOpr::NotEquals),
            '<' => Some(TexMathOpr::LessThan),
            '>' => Some(TexMathOpr::GreaterThan),
            '≤' => Some(TexMathOpr::LessEq),
            '≥' => Some(TexMathOpr::GreaterEq),
            '±' => Some(TexMathOpr::PlusMinus),
            '×' => Some(TexMathOpr::Times),
            _ => None,
        }
    }
}
