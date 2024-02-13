use super::*;

impl<'a> AstParser<'a> {
    pub(super) fn indent(&self) -> u32 {
        self.indent.0
    }

    /// do something with indent increased
    pub(super) fn indented(&mut self, f: impl FnOnce(&mut Self) -> AstIdxRange) -> AstIdxRange {
        self.indent.incr();
        let result = f(self);
        self.indent.decr();
        result
    }

    pub(super) fn with_indent(
        &mut self,
        indent: u32,
        f: impl FnOnce(&mut Self) -> AstIdxRange,
    ) -> AstIdxRange {
        let old_indent = std::mem::replace(&mut self.indent, Indent(indent));
        let result = f(self);
        self.indent = old_indent;
        result
    }
}
