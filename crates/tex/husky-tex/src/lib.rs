//! The compiler for the _Tex_ markup language.
//!
//! # Steps
//! - **Parsing:**
//!   The compiler first transforms a plain string into an iterator of [tokens].
//!   This token stream is [parsed] into a [syntax tree]. The tree itself is
//!   untyped, but the [AST] module provides a typed layer over it.
//! - **Evaluation:**
//!   The next step is to [evaluate] the markup. This produces a [module],
//!   consisting of a scope of values that were exported by the code and
//!   [content], a hierarchical, styled representation of what was written in
//!   the source file. The elements of the content tree are well structured and
//!   order-independent and thus much better suited for further processing than
//!   the raw markup.
//! - **Layouting:**
//!   Next, the content is [layouted] into a [document] containing one [frame]
//!   per page with items at fixed positions.
//! - **Exporting:**
//!   These frames can finally be exported into an output format (currently PDF,
//!   PNG, or SVG).
//!
//! [tokens]: syntax::SyntaxKind
//! [parsed]: syntax::parse
//! [syntax tree]: syntax::SyntaxNode
//! [AST]: syntax::ast
//! [evaluate]: eval::eval
//! [module]: foundations::Module
//! [content]: foundations::TexContent
//! [layouted]: layout::LayoutRoot
//! [document]: model::Document
//! [frame]: layout::Frame

#![recursion_limit = "1000"]
#![allow(clippy::comparison_chain)]
#![allow(clippy::wildcard_in_or_patterns)]
#![allow(clippy::manual_range_contains)]

extern crate self as husky_tex;

#[macro_use]
pub mod util;
pub mod diag;
pub mod engine;
pub mod eval;
pub mod foundations;
pub mod introspection;
pub mod layout;
pub mod loading;
pub mod math;
pub mod model;
pub mod realize;
pub mod symbols;
pub mod text;
pub mod visualize;

#[doc(inline)]
pub use husky_tex_syntax as syntax;

use crate::diag::{warning, FileResult, SourceDiagnostic, SourceResult};
use crate::engine::{Engine, Route};
use crate::eval::Tracer;
use crate::foundations::{
    Array, Bytes, Datetime, Module, Scope, StyleChain, Styles, TexContent, TexDict,
};
use crate::introspection::{Introspector, Locator};
use crate::layout::{LayoutRoot, TexAlignment, TexLayoutDirection};
use crate::model::TexDocument;
use crate::syntax::{FileId, PackageSpec, Source, Span};
use crate::text::{TexFont, TexFontBook};
use crate::visualize::TexColor;
use comemo::{Prehashed, Track, Tracked, Validate};
use ecow::{EcoString, EcoVec};
use husky_tex_timing::{timed, TimingScope};
use std::collections::HashSet;
use std::ops::Range;

/// Compile a source file into a fully layouted document.
///
/// - Returns `Ok(document)` if there were no fatal errors.
/// - Returns `Err(errors)` if there were fatal errors.
///
/// Requires a mutable reference to a tracer. Such a tracer can be created with
/// `Tracer::new()`. Independently of whether compilation succeeded, calling
/// `tracer.warnings()` after compilation will return all compiler warnings.
#[husky_tex_macros::time(name = "compile")]
pub fn compile(world: &dyn World, tracer: &mut Tracer) -> SourceResult<TexDocument> {
    // Call `track` on the world just once to keep comemo's ID stable.
    let world = world.track();

    // Try to evaluate the source file into a module.
    let module = crate::eval::eval(
        world,
        Route::default().track(),
        tracer.track_mut(),
        &world.main(),
    )
    .map_err(deduplicate)?;
    use husky_print_utils::p;
    p!(module);

    // Typeset the module's content, relayouting until convergence.
    typeset(world, tracer, &module.content()).map_err(deduplicate)
}

/// Relayout until introspection converges.
fn typeset(
    world: Tracked<dyn World + '_>,
    tracer: &mut Tracer,
    content: &TexContent,
) -> SourceResult<TexDocument> {
    // The name of the iterations for timing scopes.
    const ITER_NAMES: &[&str] = &[
        "typeset (1)",
        "typeset (2)",
        "typeset (3)",
        "typeset (4)",
        "typeset (5)",
    ];

    let library = world.library();
    let styles = StyleChain::new(&library.styles);

    let mut iter = 0;
    let mut document = TexDocument::default();

    // Relayout until all introspections stabilize.
    // If that doesn't happen within five attempts, we give up.
    loop {
        let _scope = TimingScope::new(ITER_NAMES[iter], None);

        // Clear delayed errors.
        tracer.delayed();

        let constraint = <Introspector as Validate>::Constraint::new();
        let mut locator = Locator::new();
        let mut engine = Engine {
            world,
            route: Route::default(),
            tracer: tracer.track_mut(),
            locator: &mut locator,
            introspector: document.introspector.track_with(&constraint),
        };

        // Layout!
        document = content.layout_root(&mut engine, styles)?;
        document.introspector.rebuild(&document.pages);
        iter += 1;

        if timed!(
            "check stabilized",
            document.introspector.validate(&constraint)
        ) {
            break;
        }

        if iter >= 5 {
            tracer.warn(warning!(
                Span::detached(), "layout did not converge within 5 attempts";
                hint: "check if any states or queries are updating themselves"
            ));
            break;
        }
    }

    // Promote delayed errors.
    let delayed = tracer.delayed();
    if !delayed.is_empty() {
        return Err(delayed);
    }

    Ok(document)
}

/// Deduplicate diagnostics.
fn deduplicate(mut diags: EcoVec<SourceDiagnostic>) -> EcoVec<SourceDiagnostic> {
    let mut unique = HashSet::new();
    diags.retain(|diag| {
        let hash = crate::util::hash128(&(&diag.span, &diag.message));
        unique.insert(hash)
    });
    diags
}

/// The environment in which typesetting occurs.
///
/// All loading functions (`main`, `source`, `file`, `font`) should perform
/// internal caching so that they are relatively cheap on repeated invocations
/// with the same argument. [`Source`], [`Bytes`], and [`Font`] are
/// all reference-counted and thus cheap to clone.
///
/// The compiler doesn't do the caching itself because the world has much more
/// information on when something can change. For example, fonts typically don't
/// change and can thus even be cached across multiple compilations (for
/// long-running applications like `tex watch`). Source files on the other
/// hand can change and should thus be cleared after each compilation. Advanced
/// clients like language servers can also retain the source files and
/// [edit](Source::edit) them in-place to benefit from better incremental
/// performance.
#[comemo::track]
pub trait World {
    /// The standard library.
    ///
    /// Can be created through `Library::build()`.
    fn library(&self) -> &Prehashed<Library>;

    /// Metadata about all known fonts.
    fn book(&self) -> &Prehashed<TexFontBook>;

    /// Access the main source file.
    fn main(&self) -> Source;

    /// Try to access the specified source file.
    fn source(&self, id: FileId) -> FileResult<Source>;

    /// Try to access the specified file.
    fn file(&self, id: FileId) -> FileResult<Bytes>;

    /// Try to access the font with the given index in the font book.
    fn font(&self, index: usize) -> Option<TexFont>;

    /// Get the current date.
    ///
    /// If no offset is specified, the local date should be chosen. Otherwise,
    /// the UTC date should be chosen with the corresponding offset in hours.
    ///
    /// If this function returns `None`, Tex's `datetime` function will
    /// return an error.
    fn today(&self, offset: Option<i64>) -> Option<Datetime>;

    /// A list of all available packages and optionally descriptions for them.
    ///
    /// This function is optional to implement. It enhances the user experience
    /// by enabling autocompletion for packages. Details about packages from the
    /// `@preview` namespace are available from
    /// `https://packages.tex.org/preview/index.json`.
    fn packages(&self) -> &[(PackageSpec, Option<EcoString>)] {
        &[]
    }
}

/// Helper methods on [`World`] implementations.
pub trait WorldExt {
    /// Get the byte range for a span.
    ///
    /// Returns `None` if the `Span` does not point into any source file.
    fn range(&self, span: Span) -> Option<Range<usize>>;
}

impl<T: World> WorldExt for T {
    fn range(&self, span: Span) -> Option<Range<usize>> {
        self.source(span.id()?).ok()?.range(span)
    }
}

/// Definition of Tex's standard library.
#[derive(Debug, Clone, Hash)]
pub struct Library {
    /// The module that contains the definitions that are available everywhere.
    pub global: Module,
    /// The module that contains the definitions available in math mode.
    pub math: Module,
    /// The default style properties (for page size, font selection, and
    /// everything else configurable via set and show rules).
    pub styles: Styles,
}

impl Library {
    /// Create a new builder for a library.
    pub fn builder() -> LibraryBuilder {
        LibraryBuilder::default()
    }
}

impl Default for Library {
    /// Constructs the standard library with the default configuration.
    fn default() -> Self {
        Self::builder().build()
    }
}

/// Configurable builder for the standard library.
///
/// This struct is created by [`Library::builder`].
#[derive(Debug, Clone, Default)]
pub struct LibraryBuilder {
    inputs: Option<TexDict>,
}

impl LibraryBuilder {
    /// Configure the inputs visible through `sys.inputs`.
    pub fn with_inputs(mut self, inputs: TexDict) -> Self {
        self.inputs = Some(inputs);
        self
    }

    /// Consumes the builder and returns a `Library`.
    pub fn build(self) -> Library {
        let math = math::module();
        let inputs = self.inputs.unwrap_or_default();
        let global = global(math.clone(), inputs);
        Library {
            global,
            math,
            styles: Styles::new(),
        }
    }
}

/// Construct the module with global definitions.
fn global(math: Module, inputs: TexDict) -> Module {
    let mut global = Scope::deduplicating();
    self::foundations::define(&mut global, inputs);
    self::model::define(&mut global);
    self::text::define(&mut global);
    global.reset_category();
    global.define_module(math);
    self::layout::define(&mut global);
    self::visualize::define(&mut global);
    self::introspection::define(&mut global);
    self::loading::define(&mut global);
    self::symbols::define(&mut global);
    prelude(&mut global);
    Module::new("global", global)
}

/// Defines scoped values that are globally available, too.
fn prelude(global: &mut Scope) {
    global.reset_category();
    global.define("black", TexColor::BLACK);
    global.define("gray", TexColor::GRAY);
    global.define("silver", TexColor::SILVER);
    global.define("white", TexColor::WHITE);
    global.define("navy", TexColor::NAVY);
    global.define("blue", TexColor::BLUE);
    global.define("aqua", TexColor::AQUA);
    global.define("teal", TexColor::TEAL);
    global.define("eastern", TexColor::EASTERN);
    global.define("purple", TexColor::PURPLE);
    global.define("fuchsia", TexColor::FUCHSIA);
    global.define("maroon", TexColor::MAROON);
    global.define("red", TexColor::RED);
    global.define("orange", TexColor::ORANGE);
    global.define("yellow", TexColor::YELLOW);
    global.define("olive", TexColor::OLIVE);
    global.define("green", TexColor::GREEN);
    global.define("lime", TexColor::LIME);
    global.define("luma", TexColor::luma_data());
    global.define("oklab", TexColor::oklab_data());
    global.define("oklch", TexColor::oklch_data());
    global.define("rgb", TexColor::rgb_data());
    global.define("cmyk", TexColor::cmyk_data());
    global.define("range", Array::range_data());
    global.define("ltr", TexLayoutDirection::LeftRight);
    global.define("rtl", TexLayoutDirection::RightLeft);
    global.define("ttb", TexLayoutDirection::TopDown);
    global.define("btt", TexLayoutDirection::BottomUp);
    global.define("start", TexAlignment::START);
    global.define("left", TexAlignment::LEFT);
    global.define("center", TexAlignment::CENTER);
    global.define("right", TexAlignment::RIGHT);
    global.define("end", TexAlignment::END);
    global.define("top", TexAlignment::TOP);
    global.define("horizon", TexAlignment::HORIZON);
    global.define("bottom", TexAlignment::BOTTOM);
}
