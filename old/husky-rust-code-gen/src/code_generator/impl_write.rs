use super::*;
use std::fmt::Write;

impl<'a> RustCodeGenerator<'a> {
    pub(crate) fn write<T: std::fmt::Display>(&mut self, s: T) {
        write!(self.result, "{}", s).unwrap()
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
