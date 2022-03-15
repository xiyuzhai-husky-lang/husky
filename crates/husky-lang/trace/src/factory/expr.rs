#[derive(Debug)]
pub struct ExprTokenConfig {
    pub(super) associated: bool,
    pub(super) appended: bool,
}

impl ExprTokenConfig {
    pub(super) fn stmt() -> Self {
        Self {
            associated: true,
            appended: true,
        }
    }

    pub(super) fn exec() -> Self {
        Self {
            associated: true,
            appended: false,
        }
    }

    pub(super) fn branch() -> Self {
        Self {
            associated: true,
            appended: true,
        }
    }

    pub(super) fn expr() -> Self {
        Self {
            associated: false,
            appended: true,
        }
    }

    pub(super) fn subexpr(&self) -> Self {
        Self {
            associated: self.associated,
            appended: false,
        }
    }

    pub(super) fn loop_head() -> Self {
        Self {
            associated: false,
            appended: false,
        }
    }
}
