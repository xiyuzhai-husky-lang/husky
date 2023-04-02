const INDENT_INCR: u32 = 4;

pub(crate) struct Indent(u32);

impl Indent {
    pub fn incr(self) -> Self {
        Self(self.0 + INDENT_INCR)
    }
}
