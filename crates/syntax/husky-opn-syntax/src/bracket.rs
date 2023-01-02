#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Bracket {
    Par,
    Box,
    Angle,
    Curl,
    Vertical,
}

impl Bracket {
    pub fn bra_code(&self) -> &'static str {
        match self {
            Bracket::Par => "(",
            Bracket::Box => "[",
            Bracket::Angle => "<",
            Bracket::Curl => "{",
            Bracket::Vertical => "|",
        }
    }

    pub fn ket_code(&self) -> &'static str {
        match self {
            Bracket::Par => ")",
            Bracket::Box => "]",
            Bracket::Angle => ">",
            Bracket::Curl => "}",
            Bracket::Vertical => "|",
        }
    }
}
