use super::*;

impl<'a> RustGenerator<'a> {
    pub(super) fn write(&mut self, s: &str) {
        self.result += s
    }

    pub(super) fn write_indent(&mut self) {
        for _ in 0..self.indent {
            self.result.push(' ');
        }
    }

    pub(super) fn write_newline(&mut self) {
        self.result.push('\n');
    }
}
