use crate::{
    error::TypstEquationResult, fragment::MathFragment, row::MathRow, StyleChain, TypstLayoutMath,
};

pub(crate) struct MathContext {
    fragments: Vec<MathFragment>,
}

impl MathContext {
    pub fn push(&mut self, fragment: impl Into<MathFragment>) {
        self.fragments.push(fragment.into());
    }

    pub fn layout_root(
        &mut self,
        elem: &dyn TypstLayoutMath,
        styles: StyleChain,
    ) -> TypstEquationResult<MathRow> {
        let row = self.layout_fragments(elem, styles)?;
        Ok(MathRow::new(row))
    }

    pub fn layout_fragment(
        &mut self,
        elem: &dyn TypstLayoutMath,
        styles: StyleChain,
    ) -> TypstEquationResult<MathFragment> {
        let row = self.layout_fragments(elem, styles)?;
        Ok(MathRow::new(row).into_fragment(self, styles))
    }

    pub fn layout_fragments(
        &mut self,
        elem: &dyn TypstLayoutMath,
        styles: StyleChain,
    ) -> TypstEquationResult<Vec<MathFragment>> {
        let prev = std::mem::take(&mut self.fragments);
        elem.layout_math(self, styles)?;
        Ok(std::mem::replace(&mut self.fragments, prev))
    }
}
