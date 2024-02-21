//! Evaluation of markup and code.

pub(crate) mod ops;

mod access;
mod binding;
mod call;
mod code;
mod flow;
mod import;
mod markup;
mod math;
mod rules;
mod tracer;
mod vm;

pub use self::call::*;
pub use self::import::*;
pub use self::tracer::*;
pub use self::vm::*;

pub(crate) use self::access::*;
pub(crate) use self::binding::*;
pub(crate) use self::flow::*;

use comemo::{Track, Tracked, TrackedMut};

use crate::diag::{bail, SourceResult};
use crate::engine::{Route, TypstEngine};
use crate::foundations::{
    Cast, IsTypstElem, TypstModuleEvaluation, TypstValue, TypstValueAssignmentGroup,
    TypstValueAssignmentGroups,
};
use crate::introspection::{Introspector, Locator};
use crate::math::EquationTypstElem;
use crate::syntax::{ast, parse, parse_code, parse_math, Source, TypstSynSpan};
use crate::IsTypstWorld;

/// Evaluate a source file and return the resulting module.
#[comemo::memoize]
#[husky_typst_macros::time(name = "eval", span = source.root().span())]
pub fn eval(
    world: Tracked<dyn IsTypstWorld + '_>,
    route: Tracked<Route>,
    tracer: TrackedMut<Tracer>,
    source: &Source,
) -> SourceResult<TypstModuleEvaluation> {
    // Prevent cyclic evaluation.
    let id = source.id();
    if route.contains(id) {
        panic!("Tried to cyclicly evaluate {:?}", id.vpath());
    }

    // Prepare the engine.
    let mut locator = Locator::new();
    let introspector = Introspector::default();
    let engine = TypstEngine {
        world,
        route: Route::extend(route).with_id(id),
        introspector: introspector.track(),
        locator: &mut locator,
        tracer,
    };

    // Prepare VM.
    let root = source.root();
    let scopes = TypstValueAssignmentGroups::new(Some(world.library()));
    let mut vm = Vm::new(engine, scopes, root.span());

    // Check for well-formedness unless we are in trace mode.
    let errors = root.errors();
    if !errors.is_empty() && vm.inspected.is_none() {
        return Err(errors.into_iter().map(Into::into).collect());
    }

    // Evaluate the module.
    let markup = root.cast::<ast::TypstMarkup>().unwrap();
    let output = markup.eval(&mut vm)?;

    // Handle control flow.
    if let Some(flow) = vm.flow {
        bail!(flow.forbidden());
    }

    // Assemble the module.
    let name = id
        .vpath()
        .as_rootless_path()
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy();

    Ok(TypstModuleEvaluation::new(name, vm.scopes.top).with_content(output))
}

/// Evaluate a string as code and return the resulting value.
///
/// Everything in the output is associated with the given `span`.
#[comemo::memoize]
pub fn eval_string(
    world: Tracked<dyn IsTypstWorld + '_>,
    string: &str,
    span: TypstSynSpan,
    mode: EvalMode,
    scope: TypstValueAssignmentGroup,
) -> SourceResult<TypstValue> {
    let mut root = match mode {
        EvalMode::Code => parse_code(string),
        EvalMode::TypstMarkup => parse(string),
        EvalMode::Math => parse_math(string),
    };

    root.synthesize(span);

    // Check for well-formedness.
    let errors = root.errors();
    if !errors.is_empty() {
        return Err(errors.into_iter().map(Into::into).collect());
    }

    // Prepare the engine.
    let mut tracer = Tracer::new();
    let mut locator = Locator::new();
    let introspector = Introspector::default();
    let engine = TypstEngine {
        world,
        introspector: introspector.track(),
        route: Route::default(),
        locator: &mut locator,
        tracer: tracer.track_mut(),
    };

    // Prepare VM.
    let scopes = TypstValueAssignmentGroups::new(Some(world.library()));
    let mut vm = Vm::new(engine, scopes, root.span());
    vm.scopes.scopes.push(scope);

    // Evaluate the code.
    let output = match mode {
        EvalMode::Code => root.cast::<ast::Code>().unwrap().eval(&mut vm)?,
        EvalMode::TypstMarkup => {
            TypstValue::Content(root.cast::<ast::TypstMarkup>().unwrap().eval(&mut vm)?)
        }
        EvalMode::Math => TypstValue::Content(
            EquationTypstElem::new(root.cast::<ast::Math>().unwrap().eval(&mut vm)?)
                .with_block(false)
                .pack(),
        ),
    };

    // Handle control flow.
    if let Some(flow) = vm.flow {
        bail!(flow.forbidden());
    }

    Ok(output)
}

/// In which mode to evaluate a string.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Cast)]
pub enum EvalMode {
    /// Evaluate as code, as after a hash.
    Code,
    /// Evaluate as markup, like in a Typst file.
    TypstMarkup,
    /// Evaluate as math, as in an equation.
    Math,
}

/// Evaluate an expression.
pub trait Eval {
    /// The output of evaluating the expression.
    type Output;

    /// Evaluate the expression to the output value.
    fn eval(self, vm: &mut Vm) -> SourceResult<Self::Output>;
}
