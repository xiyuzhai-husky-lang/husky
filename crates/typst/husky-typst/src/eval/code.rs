use ecow::{eco_vec, EcoVec};

use crate::diag::{bail, error, At, SourceDiagnostic, SourceResult};
use crate::eval::{ops, Eval, Vm};
use crate::foundations::{Array, Str, TypstContent, TypstDict, TypstValue};
use crate::syntax::ast::{self, TypstAstNode};

impl Eval for ast::Code<'_> {
    type Output = TypstValue;

    fn eval(self, vm: &mut Vm) -> SourceResult<Self::Output> {
        eval_code(vm, &mut self.exprs())
    }
}

/// Evaluate a stream of expressions.
fn eval_code<'a>(
    vm: &mut Vm,
    exprs: &mut impl Iterator<Item = ast::Expr<'a>>,
) -> SourceResult<TypstValue> {
    let flow = vm.flow.take();
    let mut output = TypstValue::None;

    while let Some(expr) = exprs.next() {
        let span = expr.span();
        let value = match expr {
            ast::Expr::Set(set) => {
                let styles = set.eval(vm)?;
                if vm.flow.is_some() {
                    break;
                }

                let tail = eval_code(vm, exprs)?.display();
                TypstValue::Content(tail.styled_with_map(styles))
            }
            ast::Expr::Show(show) => {
                let recipe = show.eval(vm)?;
                if vm.flow.is_some() {
                    break;
                }

                let tail = eval_code(vm, exprs)?.display();
                TypstValue::Content(tail.styled_with_recipe(&mut vm.engine, recipe)?)
            }
            _ => expr.eval(vm)?,
        };

        output = ops::join(output, value).at(span)?;

        if vm.flow.is_some() {
            break;
        }
    }

    if flow.is_some() {
        vm.flow = flow;
    }

    Ok(output)
}

impl Eval for ast::Expr<'_> {
    type Output = TypstValue;

    fn eval(self, vm: &mut Vm) -> SourceResult<Self::Output> {
        let span = self.span();
        let forbidden = |name| {
            error!(
                span,
                "{} is only allowed directly in code and content blocks", name
            )
        };

        let v = match self {
            Self::Text(v) => v.eval(vm).map(TypstValue::Content),
            Self::Space(v) => v.eval(vm).map(TypstValue::Content),
            Self::Linebreak(v) => v.eval(vm).map(TypstValue::Content),
            Self::Parbreak(v) => v.eval(vm).map(TypstValue::Content),
            Self::Escape(v) => v.eval(vm),
            Self::Shorthand(v) => v.eval(vm),
            Self::SmartQuote(v) => v.eval(vm).map(TypstValue::Content),
            Self::Strong(v) => v.eval(vm).map(TypstValue::Content),
            Self::Emph(v) => v.eval(vm).map(TypstValue::Content),
            Self::Raw(v) => v.eval(vm).map(TypstValue::Content),
            Self::Link(v) => v.eval(vm).map(TypstValue::Content),
            Self::Label(v) => v.eval(vm),
            Self::Ref(v) => v.eval(vm).map(TypstValue::Content),
            Self::Heading(v) => v.eval(vm).map(TypstValue::Content),
            Self::List(v) => v.eval(vm).map(TypstValue::Content),
            Self::Enum(v) => v.eval(vm).map(TypstValue::Content),
            Self::Term(v) => v.eval(vm).map(TypstValue::Content),
            Self::Equation(v) => v.eval(vm).map(TypstValue::Content),
            Self::Math(v) => v.eval(vm).map(TypstValue::Content),
            Self::MathIdent(v) => v.eval(vm),
            Self::MathAlignPoint(v) => v.eval(vm).map(TypstValue::Content),
            Self::MathDelimited(v) => v.eval(vm).map(TypstValue::Content),
            Self::MathAttach(v) => v.eval(vm).map(TypstValue::Content),
            Self::MathPrimes(v) => v.eval(vm).map(TypstValue::Content),
            Self::MathFrac(v) => v.eval(vm).map(TypstValue::Content),
            Self::MathRoot(v) => v.eval(vm).map(TypstValue::Content),
            Self::Ident(v) => v.eval(vm),
            Self::None(v) => v.eval(vm),
            Self::Auto(v) => v.eval(vm),
            Self::Bool(v) => v.eval(vm),
            Self::Int(v) => v.eval(vm),
            Self::Float(v) => v.eval(vm),
            Self::Numeric(v) => v.eval(vm),
            Self::Str(v) => v.eval(vm),
            Self::Code(v) => v.eval(vm),
            Self::Content(v) => v.eval(vm).map(TypstValue::Content),
            Self::Array(v) => v.eval(vm).map(TypstValue::Array),
            Self::Dict(v) => v.eval(vm).map(TypstValue::Dict),
            Self::Parenthesized(v) => v.eval(vm),
            Self::FieldAccess(v) => v.eval(vm),
            Self::FuncCall(v) => v.eval(vm),
            Self::Closure(v) => v.eval(vm),
            Self::Unary(v) => v.eval(vm),
            Self::Binary(v) => v.eval(vm),
            Self::Let(v) => v.eval(vm),
            Self::DestructAssign(v) => v.eval(vm),
            Self::Set(_) => bail!(forbidden("set")),
            Self::Show(_) => bail!(forbidden("show")),
            Self::Conditional(v) => v.eval(vm),
            Self::While(v) => v.eval(vm),
            Self::For(v) => v.eval(vm),
            Self::Import(v) => v.eval(vm),
            Self::Include(v) => v.eval(vm).map(TypstValue::Content),
            Self::Break(v) => v.eval(vm),
            Self::Continue(v) => v.eval(vm),
            Self::Return(v) => v.eval(vm),
        }?
        .spanned(span);

        if vm.inspected == Some(span) {
            vm.engine.tracer.value(v.clone());
        }

        Ok(v)
    }
}

impl Eval for ast::Ident<'_> {
    type Output = TypstValue;

    fn eval(self, vm: &mut Vm) -> SourceResult<Self::Output> {
        vm.scopes.get(&self).cloned().at(self.span())
    }
}

impl Eval for ast::None<'_> {
    type Output = TypstValue;

    fn eval(self, _: &mut Vm) -> SourceResult<Self::Output> {
        Ok(TypstValue::None)
    }
}

impl Eval for ast::Auto<'_> {
    type Output = TypstValue;

    fn eval(self, _: &mut Vm) -> SourceResult<Self::Output> {
        Ok(TypstValue::Auto)
    }
}

impl Eval for ast::Bool<'_> {
    type Output = TypstValue;

    fn eval(self, _: &mut Vm) -> SourceResult<Self::Output> {
        Ok(TypstValue::Bool(self.get()))
    }
}

impl Eval for ast::Int<'_> {
    type Output = TypstValue;

    fn eval(self, _: &mut Vm) -> SourceResult<Self::Output> {
        Ok(TypstValue::Int(self.get()))
    }
}

impl Eval for ast::Float<'_> {
    type Output = TypstValue;

    fn eval(self, _: &mut Vm) -> SourceResult<Self::Output> {
        Ok(TypstValue::Float(self.get()))
    }
}

impl Eval for ast::Numeric<'_> {
    type Output = TypstValue;

    fn eval(self, _: &mut Vm) -> SourceResult<Self::Output> {
        Ok(TypstValue::numeric(self.get()))
    }
}

impl Eval for ast::Str<'_> {
    type Output = TypstValue;

    fn eval(self, _: &mut Vm) -> SourceResult<Self::Output> {
        Ok(TypstValue::Str(self.get().into()))
    }
}

impl Eval for ast::Array<'_> {
    type Output = Array;

    fn eval(self, vm: &mut Vm) -> SourceResult<Self::Output> {
        let items = self.items();

        let mut vec = EcoVec::with_capacity(items.size_hint().0);
        for item in items {
            match item {
                ast::ArrayItem::Pos(expr) => vec.push(expr.eval(vm)?),
                ast::ArrayItem::Spread(expr) => match expr.eval(vm)? {
                    TypstValue::None => {}
                    TypstValue::Array(array) => vec.extend(array.into_iter()),
                    v => bail!(expr.span(), "cannot spread {} into array", v.ty()),
                },
            }
        }

        Ok(vec.into())
    }
}

impl Eval for ast::Dict<'_> {
    type Output = TypstDict;

    fn eval(self, vm: &mut Vm) -> SourceResult<Self::Output> {
        let mut map = indexmap::IndexMap::new();

        let mut invalid_keys = eco_vec![];

        for item in self.items() {
            match item {
                ast::DictItem::Named(named) => {
                    map.insert(named.name().get().clone().into(), named.expr().eval(vm)?);
                }
                ast::DictItem::Keyed(keyed) => {
                    let raw_key = keyed.key();
                    let key = raw_key.eval(vm)?;
                    let key = key.cast::<Str>().unwrap_or_else(|error| {
                        let error = SourceDiagnostic::error(raw_key.span(), error);
                        invalid_keys.push(error);
                        Str::default()
                    });
                    map.insert(key, keyed.expr().eval(vm)?);
                }
                ast::DictItem::Spread(expr) => match expr.eval(vm)? {
                    TypstValue::None => {}
                    TypstValue::Dict(dict) => map.extend(dict.into_iter()),
                    v => bail!(expr.span(), "cannot spread {} into dictionary", v.ty()),
                },
            }
        }

        if !invalid_keys.is_empty() {
            return Err(invalid_keys);
        }

        Ok(map.into())
    }
}

impl Eval for ast::CodeBlock<'_> {
    type Output = TypstValue;

    fn eval(self, vm: &mut Vm) -> SourceResult<Self::Output> {
        vm.scopes.enter();
        let output = self.body().eval(vm)?;
        vm.scopes.exit();
        Ok(output)
    }
}

impl Eval for ast::ContentBlock<'_> {
    type Output = TypstContent;

    fn eval(self, vm: &mut Vm) -> SourceResult<Self::Output> {
        vm.scopes.enter();
        let content = self.body().eval(vm)?;
        vm.scopes.exit();
        Ok(content)
    }
}

impl Eval for ast::Parenthesized<'_> {
    type Output = TypstValue;

    fn eval(self, vm: &mut Vm) -> SourceResult<Self::Output> {
        self.expr().eval(vm)
    }
}

impl Eval for ast::FieldAccess<'_> {
    type Output = TypstValue;

    fn eval(self, vm: &mut Vm) -> SourceResult<Self::Output> {
        let value = self.target().eval(vm)?;
        let field = self.field();
        value.field(&field).at(field.span())
    }
}
