use latex_command::path::menu::{command_path_menu, LxCommandPathMenu};
use latex_math_letter::letter::LxMathLetter;
use latex_math_punctuation::{LxMathPunctuation, LxMathPunctuationMap};
use visored_item_path::path::VdItemPath;

use crate::{
    resolution::{
        command::{VdCompleteCommandResolution, VdCompleteCommandResolutionMap},
        letter::{VdLetterResolution, VdLetterResolutionMap},
        punctuation::VdPunctuationResolution,
    },
    table::VdDefaultResolutionTable,
};

impl VdDefaultResolutionTable {
    pub fn new_standard(db: &salsa::Db) -> Self {
        let punctuation_resolution_map =
            LxMathPunctuationMap::new(lx_math_punctuation_standard_resolution);
        let command_resolution_map = standard_command_resolution_map(db);
        let letter_resolution_map = standard_letter_resolution_map(db);
        Self::new(
            punctuation_resolution_map,
            command_resolution_map,
            letter_resolution_map,
        )
    }
}

fn standard_command_resolution_map(
    db: &salsa::Db,
) -> std::collections::HashMap<
    latex_command::path::LxCommandPath,
    crate::resolution::command::VdCompleteCommandResolution,
    rustc_hash::FxBuildHasher,
> {
    let LxCommandPathMenu {
        begin: _,
        end: _,
        // maths
        // - letter style
        mathbb,
        mathbf,
        mathcal,
        mathit,
        mathrm,
        mathsf,
        mathscr,
        // - operators
        // -- relations
        eq,
        ne,
        r#in,
        subset,
        supset,
        subseteq,
        supseteq,
        subseteqq,
        supseteqq,
        subsetneq,
        supsetneq,
        // -- arithmetic
        int,
        sum,
        times,
        otimes,
        prod,
        // - extended letters
        alpha,
        beta,
        gamma,
        pi,
        // - functions
        sin,
        cos,
        // - layouts
        sqrt,
        frac,
        // - environments
        text,
    } = *command_path_menu(db);
    VdCompleteCommandResolutionMap::from_iter([
        // - operators
        // -- relations
        (eq, VdCompleteCommandResolution::EQ),
        (ne, VdCompleteCommandResolution::NE),
        (r#in, VdCompleteCommandResolution::IN),
        (subset, VdCompleteCommandResolution::SUBSET),
        (supset, VdCompleteCommandResolution::SUPSET),
        // -- arithmetic
        (int, VdCompleteCommandResolution::INT),
        (sum, VdCompleteCommandResolution::SUM),
        (prod, VdCompleteCommandResolution::PROD),
        (times, VdCompleteCommandResolution::TIMES),
        (otimes, VdCompleteCommandResolution::OTIMES),
        // - extended letters
        (alpha, VdCompleteCommandResolution::LOWER_ALPHA),
        (beta, VdCompleteCommandResolution::LOWER_BETA),
        (gamma, VdCompleteCommandResolution::LOWER_GAMMA),
        (pi, VdCompleteCommandResolution::LOWER_PI),
        (sin, VdCompleteCommandResolution::Item(VdItemPath::SIN)),
        (cos, VdCompleteCommandResolution::Item(VdItemPath::COS)),
        (sqrt, VdCompleteCommandResolution::Sqrt),
        (frac, VdCompleteCommandResolution::Frac),
        (text, VdCompleteCommandResolution::Text),
    ])
}

fn lx_math_punctuation_standard_resolution(
    punctuation: LxMathPunctuation,
) -> Option<VdPunctuationResolution> {
    match punctuation {
        LxMathPunctuation::Add => Some(VdPunctuationResolution::SEPARATOR_ADD),
        LxMathPunctuation::Sub => Some(VdPunctuationResolution::SUB),
        LxMathPunctuation::Mul => Some(VdPunctuationResolution::SEPARATOR_MUL),
        LxMathPunctuation::Div => Some(VdPunctuationResolution::Todo),
        LxMathPunctuation::In => Some(VdPunctuationResolution::Todo),
        LxMathPunctuation::NotIn => Some(VdPunctuationResolution::Todo),
        LxMathPunctuation::Subset => Some(VdPunctuationResolution::Todo),
        LxMathPunctuation::Superset => Some(VdPunctuationResolution::Todo),
        LxMathPunctuation::SubsetEq => Some(VdPunctuationResolution::Todo),
        LxMathPunctuation::SupersetEq => Some(VdPunctuationResolution::Todo),
        LxMathPunctuation::ForAll => Some(VdPunctuationResolution::Todo),
        LxMathPunctuation::Exists => Some(VdPunctuationResolution::Todo),
        LxMathPunctuation::NotExists => Some(VdPunctuationResolution::Todo),
        LxMathPunctuation::Infinity => Some(VdPunctuationResolution::Todo),
        LxMathPunctuation::Equals => Some(VdPunctuationResolution::EQ),
        LxMathPunctuation::NotEquals => Some(VdPunctuationResolution::Todo),
        LxMathPunctuation::LessThan => Some(VdPunctuationResolution::Todo),
        LxMathPunctuation::GreaterThan => Some(VdPunctuationResolution::Todo),
        LxMathPunctuation::LessEq => Some(VdPunctuationResolution::Todo),
        LxMathPunctuation::GreaterEq => Some(VdPunctuationResolution::Todo),
        LxMathPunctuation::PlusMinus => Some(VdPunctuationResolution::Todo),
        LxMathPunctuation::Times => Some(VdPunctuationResolution::Todo),
        LxMathPunctuation::Lpar => Some(VdPunctuationResolution::LPAR),
        LxMathPunctuation::Rpar => Some(VdPunctuationResolution::RPAR),
        LxMathPunctuation::Lbox => Some(VdPunctuationResolution::Todo),
        LxMathPunctuation::Rbox => Some(VdPunctuationResolution::Todo),
        LxMathPunctuation::EscapedLcurl => Some(VdPunctuationResolution::Todo),
        LxMathPunctuation::EscapedRcurl => Some(VdPunctuationResolution::Todo),
    }
}

fn standard_letter_resolution_map(db: &salsa::Db) -> VdLetterResolutionMap {
    [
        (LxMathLetter::MATHBB_N, VdLetterResolution::NATURAL_NUMBER),
        (LxMathLetter::MATHBB_Q, VdLetterResolution::RATIONAL_NUMBER),
        (LxMathLetter::MATHBB_R, VdLetterResolution::REAL_NUMBER),
        (LxMathLetter::MATHBB_C, VdLetterResolution::COMPLEX_NUMBER),
    ]
    .into_iter()
    .collect()
}
