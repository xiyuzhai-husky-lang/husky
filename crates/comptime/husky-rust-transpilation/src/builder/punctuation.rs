use super::*;

pub enum RustPunctuation {
    Bra(RustBracket),
    Ket(RustBracket),
    Eq,
    Colon,
    Dot,
    ColonColon,
    LightArrow,
    DotDot,
    DotDotEq,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RustBracket {
    Par,       // Parentheses ()
    Box,       // Box brackets []
    TurboFish, // Turbofish ::<>
    Angle,     // Angle <>
    Curl,      // Curly brackets {}
    Vertical,  // Vertical bars ||
}

impl RustBracket {
    pub(crate) fn bra_code(self) -> &'static str {
        match self {
            RustBracket::Par => "(",
            RustBracket::Box => "[",
            RustBracket::TurboFish => "::<",
            RustBracket::Angle => "<",
            RustBracket::Curl => "{",
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
            RustBracket::Vertical => "|",
        }
    }
}

impl<'a, 'b, E> RustTranspilationBuilder<'a, 'b, E> {
    pub(crate) fn punctuation(&mut self, punctuation: RustPunctuation) {
        let s = match punctuation {
            RustPunctuation::Bra(bracket) => bracket.bra_code(),
            RustPunctuation::Ket(bracket) => bracket.ket_code(),
            RustPunctuation::Eq => " = ",
            RustPunctuation::Colon => ": ",
            RustPunctuation::Dot => ".",
            RustPunctuation::ColonColon => "::",
            RustPunctuation::LightArrow => " -> ",
            RustPunctuation::DotDot => "..",
            RustPunctuation::DotDotEq => "..=",
        };
        self.write_str(s)
    }
}
