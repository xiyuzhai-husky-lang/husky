use ecow::eco_format;

use crate::diag::{At, TypstSourceResult};
use crate::eval::{Eval, Vm};
use crate::foundations::{IsTypstElem, TypstContent, TypstValue};
use crate::math::{
    AttachTypstElem, FracTypstElem, LrElem, PrimesElem, RootElem, TypstAlignPointElem,
};
use crate::syntax::ast::{self, TypstAstNode};
use crate::text::TextElem;

impl Eval for ast::Math<'_> {
    type Output = TypstContent;
    fn eval(self, vm: &mut Vm) -> TypstSourceResult<Self::Output> {
        Ok(TypstContent::sequence(
            self.exprs()
                .map(|expr| expr.eval_display(vm))
                .collect::<TypstSourceResult<Vec<_>>>()?,
        ))
    }
}

impl Eval for ast::MathIdent<'_> {
    type Output = TypstValue;

    fn eval(self, vm: &mut Vm) -> TypstSourceResult<Self::Output> {
        vm.scopes.get_in_math(&self).cloned().at(self.span())
    }
}

impl Eval for ast::MathAlignPoint<'_> {
    type Output = TypstContent;

    fn eval(self, _: &mut Vm) -> TypstSourceResult<Self::Output> {
        Ok(TypstAlignPointElem::new().pack())
    }
}

impl Eval for ast::MathDelimited<'_> {
    type Output = TypstContent;

    fn eval(self, vm: &mut Vm) -> TypstSourceResult<Self::Output> {
        let open = self.open().eval_display(vm)?;
        let body = self.body().eval(vm)?;
        let close = self.close().eval_display(vm)?;
        Ok(LrElem::new(open + body + close).pack())
    }
}

impl Eval for ast::MathAttach<'_> {
    type Output = TypstContent;

    fn eval(self, vm: &mut Vm) -> TypstSourceResult<Self::Output> {
        let base = self.base().eval_display(vm)?;
        let mut elem = AttachTypstElem::new(base);

        if let Some(expr) = self.top() {
            elem.push_t(Some(expr.eval_display(vm)?));
        } else if let Some(primes) = self.primes() {
            elem.push_t(Some(primes.eval(vm)?));
        }

        if let Some(expr) = self.bottom() {
            elem.push_b(Some(expr.eval_display(vm)?));
        }

        Ok(elem.pack())
    }
}

impl Eval for ast::MathPrimes<'_> {
    type Output = TypstContent;

    fn eval(self, _: &mut Vm) -> TypstSourceResult<Self::Output> {
        Ok(PrimesElem::new(self.count()).pack())
    }
}

impl Eval for ast::MathFrac<'_> {
    type Output = TypstContent;

    fn eval(self, vm: &mut Vm) -> TypstSourceResult<Self::Output> {
        let num = self.num().eval_display(vm)?;
        let denom = self.denom().eval_display(vm)?;
        Ok(FracTypstElem::new(num, denom).pack())
    }
}

impl Eval for ast::MathRoot<'_> {
    type Output = TypstContent;

    fn eval(self, vm: &mut Vm) -> TypstSourceResult<Self::Output> {
        let index = self.index().map(|i| TextElem::packed(eco_format!("{i}")));
        let radicand = self.radicand().eval_display(vm)?;
        Ok(RootElem::new(radicand).with_index(index).pack())
    }
}

trait ExprExt {
    fn eval_display(&self, vm: &mut Vm) -> TypstSourceResult<TypstContent>;
}

impl ExprExt for ast::Expr<'_> {
    fn eval_display(&self, vm: &mut Vm) -> TypstSourceResult<TypstContent> {
        Ok(self.eval(vm)?.display().spanned(self.span()))
    }
}
