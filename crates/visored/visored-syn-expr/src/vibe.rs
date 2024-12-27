use snl_prelude::mode::SnlMode;

#[derive(Debug, Clone, Copy)]
pub struct VdSynExprVibe {
    snl_mode: SnlMode,
}

impl VdSynExprVibe {
    pub const ROOT_CNL: Self = Self {
        snl_mode: SnlMode::Cnl,
    };

    pub const ROOT_UNL: Self = Self {
        snl_mode: SnlMode::Unl,
    };
}

impl VdSynExprVibe {
    pub fn snl_mode(&self) -> SnlMode {
        self.snl_mode
    }
}
