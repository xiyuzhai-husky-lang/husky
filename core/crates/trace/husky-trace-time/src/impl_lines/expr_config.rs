#[derive(Debug)]
pub struct ExprTokenConfig {
    pub(crate) associated: bool,
    pub(crate) appended: bool,
}

impl ExprTokenConfig {
    pub(crate) fn stmt() -> Self {
        Self {
            associated: true,
            appended: true,
        }
    }

    pub(crate) fn exec() -> Self {
        Self {
            associated: true,
            appended: false,
        }
    }

    pub(crate) fn branch() -> Self {
        Self {
            associated: true,
            appended: true,
        }
    }

    pub(crate) fn expr(associated: bool) -> Self {
        Self {
            associated,
            appended: true,
        }
    }

    pub(crate) fn subexpr(&self) -> Self {
        Self {
            associated: self.associated,
            appended: false,
        }
    }

    pub(crate) fn loop_head() -> Self {
        Self {
            associated: false,
            appended: false,
        }
    }
}
