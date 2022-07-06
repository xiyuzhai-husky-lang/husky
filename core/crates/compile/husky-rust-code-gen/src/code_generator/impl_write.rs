use super::*;

impl<'a> RustCodeGenerator<'a> {
    pub(crate) fn write(&mut self, s: &str) {
        self.result += s
    }

    pub(super) fn indent(&mut self, n: u8) {
        for _ in 0..n {
            self.result.push(' ');
        }
    }

    pub(super) fn newline(&mut self) {
        self.result.push('\n');
    }

    pub(super) fn newline_indented(&mut self, n: u8) {
        self.result.push('\n');
        self.indent(n)
    }
}
