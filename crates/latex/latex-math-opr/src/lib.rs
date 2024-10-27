#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum LxMathOpr {
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

impl LxMathOpr {
    pub fn to_char(self) -> char {
        match self {
            LxMathOpr::Add => '+',
            LxMathOpr::Sub => '-',
            LxMathOpr::Mul => '·',
            LxMathOpr::Div => '/',
            LxMathOpr::In => '∈',
            LxMathOpr::NotIn => '∉',
            LxMathOpr::Subset => '⊂',
            LxMathOpr::Superset => '⊃',
            LxMathOpr::SubsetEq => '⊆',
            LxMathOpr::SupersetEq => '⊇',
            LxMathOpr::ForAll => '∀',
            LxMathOpr::Exists => '∃',
            LxMathOpr::NotExists => '∄',
            LxMathOpr::Infinity => '∞',
            LxMathOpr::Equals => '=',
            LxMathOpr::NotEquals => '≠',
            LxMathOpr::LessThan => '<',
            LxMathOpr::GreaterThan => '>',
            LxMathOpr::LessEq => '≤',
            LxMathOpr::GreaterEq => '≥',
            LxMathOpr::PlusMinus => '±',
            LxMathOpr::Times => '×',
        }
    }

    pub fn try_from_char(c: char) -> Option<Self> {
        match c {
            '+' => Some(LxMathOpr::Add),
            '-' => Some(LxMathOpr::Sub),
            '·' => Some(LxMathOpr::Mul),
            '/' => Some(LxMathOpr::Div),
            '∈' => Some(LxMathOpr::In),
            '∉' => Some(LxMathOpr::NotIn),
            '⊂' => Some(LxMathOpr::Subset),
            '⊃' => Some(LxMathOpr::Superset),
            '⊆' => Some(LxMathOpr::SubsetEq),
            '⊇' => Some(LxMathOpr::SupersetEq),
            '∀' => Some(LxMathOpr::ForAll),
            '∃' => Some(LxMathOpr::Exists),
            '∄' => Some(LxMathOpr::NotExists),
            '∞' => Some(LxMathOpr::Infinity),
            '=' => Some(LxMathOpr::Equals),
            '≠' => Some(LxMathOpr::NotEquals),
            '<' => Some(LxMathOpr::LessThan),
            '>' => Some(LxMathOpr::GreaterThan),
            '≤' => Some(LxMathOpr::LessEq),
            '≥' => Some(LxMathOpr::GreaterEq),
            '±' => Some(LxMathOpr::PlusMinus),
            '×' => Some(LxMathOpr::Times),
            _ => None,
        }
    }
}
