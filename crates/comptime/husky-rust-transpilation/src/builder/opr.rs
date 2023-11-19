use super::*;

pub enum RustOpr {
    Bra(RustBracket),
    Ket(RustBracket),
    Assign,
    Colon,
    Dot,
    ColonColon,
    LightArrow,
    DotDot,
    DotDotEq,
    AddAssign,
    Less,
    Leq,
    Greater,
    Geq,
    Add,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RustBracket {
    Par,        // Parentheses ()
    Box,        // Box brackets []
    TurboFish,  // Turbofish ::<>
    Angle,      // Angle <>
    Curl,       // Curly brackets {}
    CurlSpaced, // Curly brackets {  }
    Vertical,   // Vertical bars ||
}

impl RustBracket {
    pub(crate) fn bra_code(self) -> &'static str {
        match self {
            RustBracket::Par => "(",
            RustBracket::Box => "[",
            RustBracket::TurboFish => "::<",
            RustBracket::Angle => "<",
            RustBracket::Curl => "{",
            RustBracket::CurlSpaced => "{ ",
            RustBracket::Vertical => "|",
        }
    }

    pub(crate) fn ket_code(self) -> &'static str {
        match self {
            RustBracket::Par => ")",
            RustBracket::Box => "]",
            RustBracket::TurboFish => ">",
            RustBracket::Angle => ">",
            RustBracket::Curl => "}",
            RustBracket::CurlSpaced => "} ",
            RustBracket::Vertical => "|",
        }
    }
}

impl<'a, 'b, E> RustTranspilationBuilder<'a, 'b, E> {
    pub(crate) fn opr(&mut self, opr: RustOpr) {
        let s = match opr {
            RustOpr::Bra(bracket) => bracket.bra_code(),
            RustOpr::Ket(bracket) => bracket.ket_code(),
            RustOpr::Assign => " = ",
            RustOpr::Colon => ": ",
            RustOpr::Dot => ".",
            RustOpr::ColonColon => "::",
            RustOpr::LightArrow => " -> ",
            RustOpr::DotDot => "..",
            RustOpr::DotDotEq => "..=",
            RustOpr::AddAssign => " += ",
            RustOpr::Less => " < ",
            RustOpr::Leq => " <= ",
            RustOpr::Greater => "> ",
            RustOpr::Geq => " >= ",
            RustOpr::Add => " + ",
        };
        self.write_str(s)
    }
}

use super::*;
use husky_opr::{BinaryClosedOpr, BinaryComparisonOpr, BinaryShiftOpr, BinaryShortcuitLogicOpr};

impl<E> TranspileToRust<E> for HirBinaryOpr {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder<E>) {
        match self {
            HirBinaryOpr::Closed(opr) => opr.transpile_to_rust(builder),
            HirBinaryOpr::Shift(opr) => opr.transpile_to_rust(builder),
            HirBinaryOpr::Assign => builder.write_str(" = "),
            HirBinaryOpr::AssignClosed(opr) => {
                let s = match opr {
                    BinaryClosedOpr::Add => " += ",
                    BinaryClosedOpr::BitAnd => " &= ",
                    BinaryClosedOpr::BitOr => " |= ",
                    BinaryClosedOpr::BitXor => " ^= ",
                    BinaryClosedOpr::Div => " /= ",
                    BinaryClosedOpr::Mul => " *= ",
                    BinaryClosedOpr::RemEuclid => " %= ",
                    BinaryClosedOpr::Power => " **= ",
                    BinaryClosedOpr::Sub => " -= ",
                };
                builder.write_str(s)
            }
            HirBinaryOpr::AssignShift(opr) => opr.transpile_to_rust(builder),
            HirBinaryOpr::Comparison(opr) => opr.transpile_to_rust(builder),
            HirBinaryOpr::ShortCircuitLogic(opr) => opr.transpile_to_rust(builder),
            HirBinaryOpr::As => builder.write_str(" as "),
        }
    }
}

impl<E> TranspileToRust<E> for BinaryClosedOpr {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder<E>) {
        let s = match self {
            BinaryClosedOpr::Add => match builder.spaced {
                true => " + ",
                false => "+",
            },
            BinaryClosedOpr::BitAnd => match builder.spaced {
                true => " & ",
                false => "&",
            },
            BinaryClosedOpr::BitOr => match builder.spaced {
                true => " | ",
                false => "|",
            },
            BinaryClosedOpr::BitXor => match builder.spaced {
                true => " ^ ",
                false => "^",
            },
            BinaryClosedOpr::Div => match builder.spaced {
                true => " / ",
                false => "/",
            },
            BinaryClosedOpr::Mul => match builder.spaced {
                true => " * ",
                false => "*",
            },
            BinaryClosedOpr::RemEuclid => match builder.spaced {
                true => " % ",
                false => "%",
            },
            BinaryClosedOpr::Power => match builder.spaced {
                true => " ** ",
                false => "**",
            },
            BinaryClosedOpr::Sub => match builder.spaced {
                true => " - ",
                false => "-",
            },
        };
        builder.write_str(s)
    }
}

impl<E> TranspileToRust<E> for BinaryShiftOpr {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder<E>) {
        let s = match self {
            BinaryShiftOpr::Shl => match builder.spaced {
                true => " << ",
                false => "<<",
            },
            BinaryShiftOpr::Shr => match builder.spaced {
                true => " >> ",
                false => ">>",
            },
        };
        builder.write_str(s)
    }
}

impl<E> TranspileToRust<E> for BinaryComparisonOpr {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder<E>) {
        let s = match self {
            BinaryComparisonOpr::Eq => match builder.spaced {
                true => " == ",
                false => "==",
            },
            BinaryComparisonOpr::Neq => match builder.spaced {
                true => " != ",
                false => "!=",
            },
            BinaryComparisonOpr::Geq => match builder.spaced {
                true => " >= ",
                false => ">=",
            },
            BinaryComparisonOpr::Greater => match builder.spaced {
                true => " > ",
                false => ">",
            },
            BinaryComparisonOpr::Leq => match builder.spaced {
                true => " <= ",
                false => "<=",
            },
            BinaryComparisonOpr::Less => match builder.spaced {
                true => " < ",
                false => "<",
            },
        };
        builder.write_str(s)
    }
}

impl<E> TranspileToRust<E> for BinaryShortcuitLogicOpr {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder<E>) {
        let s = match self {
            BinaryShortcuitLogicOpr::And => match builder.spaced {
                true => " && ",
                false => "&&",
            },
            BinaryShortcuitLogicOpr::Or => match builder.spaced {
                true => " || ",
                false => "||",
            },
        };
        builder.write_str(s)
    }
}

impl<E> TranspileToRust<E> for HirPrefixOpr {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder<E>) {
        let s = match self {
            HirPrefixOpr::Minus => "-",
            HirPrefixOpr::Not => "!",
            HirPrefixOpr::BitNot => "!",
            HirPrefixOpr::TakeRef => "&",
            HirPrefixOpr::Deref => "*",
        };
        builder.write_str(s)
    }
}

impl<E> TranspileToRust<E> for HirSuffixOpr {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder<E>) {
        let s = match self {
            HirSuffixOpr::Incr => "+= 1",
            HirSuffixOpr::Decr => "-= 1",
            HirSuffixOpr::Unveil => "?",
            HirSuffixOpr::Unwrap => ".unwrap()",
        };
        builder.write_str(s)
    }
}
