// in husky, there is no <> bracket like in Rust or C++ by design
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Bracket {
    Par,
    Box,
    TurboFish,
    Curl,
    Vertical,
    HtmlAngle,
}

impl Bracket {
    pub fn bra_code(&self) -> &'static str {
        match self {
            Bracket::Par => "(",
            Bracket::Box => "[",
            Bracket::TurboFish => "::<",
            Bracket::Curl => "{",
            Bracket::Vertical => "|",
            Bracket::HtmlAngle => "<",
        }
    }

    pub fn ket_code(&self) -> &'static str {
        match self {
            Bracket::Par => ")",
            Bracket::Box => "]",
            Bracket::TurboFish => ">",
            Bracket::Curl => "}",
            Bracket::Vertical => "|",
            Bracket::HtmlAngle => "/>",
        }
    }
}
