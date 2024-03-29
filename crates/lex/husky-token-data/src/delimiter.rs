// in husky, there is no <> bracket like in Rust or C++ by design
#[salsa::derive_debug_with_db]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Delimiter {
    Par,
    Box,
    TurboFish,
    InlineCurl,
    NestedCurl,
    Vert,
    HtmlAngle,
}

impl Delimiter {
    pub fn bra_code(&self) -> &'static str {
        match self {
            Delimiter::Par => "(",
            Delimiter::Box => "[",
            Delimiter::TurboFish => "::<",
            Delimiter::InlineCurl | Delimiter::NestedCurl => "{",
            Delimiter::Vert => "|",
            Delimiter::HtmlAngle => "<",
        }
    }

    pub fn ket_code(&self) -> &'static str {
        match self {
            Delimiter::Par => ")",
            Delimiter::Box => "]",
            Delimiter::TurboFish => ">",
            Delimiter::InlineCurl | Delimiter::NestedCurl => "}",
            Delimiter::Vert => "|",
            Delimiter::HtmlAngle => "/>",
        }
    }
}
