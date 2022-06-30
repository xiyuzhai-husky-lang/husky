use super::*;

impl<'a> RustCodeGenerator<'a> {
    pub(super) fn write(&mut self, s: &str) {
        self.result += s
    }

    pub(super) fn indent(&mut self) {
        for _ in 0..self.indent {
            self.result.push(' ');
        }
    }

    pub(super) fn newline(&mut self) {
        self.result.push('\n');
    }

    pub(super) fn newline_indented(&mut self) {
        self.result.push('\n');
        self.indent()
    }
}
