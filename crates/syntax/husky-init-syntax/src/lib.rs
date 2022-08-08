#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InitKind {
    Let,
    Var,
    Decl,
}

impl std::fmt::Display for InitKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(match self {
            InitKind::Let => "let",
            InitKind::Var => "var",
            InitKind::Decl => "decl",
        })
    }
}
