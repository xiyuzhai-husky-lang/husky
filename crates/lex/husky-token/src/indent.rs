pub const INDENT_INCR: u32 = 4;

#[derive(Debug, Default)]
pub struct Indent(pub u32);

impl Indent {
    pub fn incr(&mut self) {
        self.0 += INDENT_INCR
    }

    pub fn decr(&mut self) {
        self.0 -= INDENT_INCR
    }
}
