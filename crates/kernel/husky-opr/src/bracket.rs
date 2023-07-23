#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Bracket {
    Par,
    Box,
    TemplateAngle,
    Curl,
    Lambda,
    HtmlAngle,
}

impl Bracket {
    pub fn bra_code(&self) -> &'static str {
        match self {
            Bracket::Par => "(",
            Bracket::Box => "[",
            Bracket::TemplateAngle => "::<",
            Bracket::Curl => "{",
            Bracket::Lambda => "|",
            Bracket::HtmlAngle => "<",
        }
    }

    pub fn ket_code(&self) -> &'static str {
        match self {
            Bracket::Par => ")",
            Bracket::Box => "]",
            Bracket::TemplateAngle => ">",
            Bracket::Curl => "}",
            Bracket::Lambda => "|",
            Bracket::HtmlAngle => "/>",
        }
    }
}
