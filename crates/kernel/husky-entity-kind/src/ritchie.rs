use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize)]
pub enum RitchieItemKind {
    Fn,
    Gn,
    Vn,
    Pn,
    Qn,
    Bn,
}

impl std::fmt::Display for RitchieItemKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.code().fmt(f)
    }
}

impl RitchieItemKind {
    pub fn code(self) -> &'static str {
        match self {
            RitchieItemKind::Fn => "fn",
            RitchieItemKind::Gn => "gn",
            RitchieItemKind::Vn => "vn",
            RitchieItemKind::Pn => "pn",
            RitchieItemKind::Qn => "qn",
            RitchieItemKind::Bn => "bn",
        }
    }

    pub fn is_lazy(self) -> bool {
        match self {
            RitchieItemKind::Fn | RitchieItemKind::Pn => false,
            RitchieItemKind::Gn
            | RitchieItemKind::Vn
            | RitchieItemKind::Qn
            | RitchieItemKind::Bn => true,
        }
    }

    pub fn needs_jar(self) -> bool {
        match self {
            RitchieItemKind::Fn => false,
            RitchieItemKind::Gn => false,
            RitchieItemKind::Vn => true,
            RitchieItemKind::Pn => false,
            RitchieItemKind::Qn => false,
            RitchieItemKind::Bn => true,
        }
    }
}
