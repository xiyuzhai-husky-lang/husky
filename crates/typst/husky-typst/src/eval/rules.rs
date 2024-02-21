use crate::diag::{At, TypstSourceResult};
use crate::eval::{Eval, Vm};
use crate::foundations::{Func, Recipe, ShowableSelector, Transformation, TypstStyles};
use crate::syntax::ast::{self, TypstAstNode};

impl Eval for ast::SetRule<'_> {
    type Output = TypstStyles;

    fn eval(self, vm: &mut Vm) -> TypstSourceResult<Self::Output> {
        if let Some(condition) = self.condition() {
            if !condition.eval(vm)?.cast::<bool>().at(condition.span())? {
                return Ok(TypstStyles::new());
            }
        }

        let target = self.target();
        let target = target
            .eval(vm)?
            .cast::<Func>()
            .and_then(|func| {
                func.element()
                    .ok_or_else(|| "only element functions can be used in set rules".into())
            })
            .at(target.span())?;
        let args = self.args().eval(vm)?;
        Ok(target.set(&mut vm.engine, args)?.spanned(self.span()))
    }
}

impl Eval for ast::ShowRule<'_> {
    type Output = Recipe;

    fn eval(self, vm: &mut Vm) -> TypstSourceResult<Self::Output> {
        let selector = self
            .selector()
            .map(|sel| sel.eval(vm)?.cast::<ShowableSelector>().at(sel.span()))
            .transpose()?
            .map(|selector| selector.0);

        let transform = self.transform();
        let span = transform.span();

        let transform = match transform {
            ast::Expr::Set(set) => Transformation::Style(set.eval(vm)?),
            expr => expr.eval(vm)?.cast::<Transformation>().at(span)?,
        };

        Ok(Recipe {
            span,
            selector,
            transform,
        })
    }
}
