use crate::diag::TypstSourceResult;
use crate::engine::TypstEngine;
use crate::foundations::{dict, func, TypstContent, TypstDict, TypstStyleChain, TypstStyles};
use crate::layout::{Axes, LayoutMultiple, Size, TypstAbsLength, TypstRegions};

/// Measures the layouted size of content.
///
/// The `measure` function lets you determine the layouted size of content. Note
/// that an infinite space is assumed, therefore the measured height/width may
/// not necessarily match the final height/width of the measured content. If you
/// want to measure in the current layout dimensions, you can combine `measure`
/// and [`layout`]($layout).
///
/// # Example
/// The same content can have a different size depending on the styles that
/// are active when it is layouted. For example, in the example below
/// `[#content]` is of course bigger when we increase the font size.
///
/// ```example
/// #let content = [Hello!]
/// #content
/// #set text(14pt)
/// #content
/// ```
///
/// To do a meaningful measurement, you therefore first need to retrieve the
/// active styles with the [`style`]($style) function. You can then pass them to
/// the `measure` function.
///
/// ```example
/// #let thing(body) = style(styles => {
///   let size = measure(body, styles)
///   [Width of "#body" is #size.width]
/// })
///
/// #thing[Hey] \
/// #thing[Welcome]
/// ```
///
/// The measure function returns a dictionary with the entries `width` and
/// `height`, both of type [`length`]($length).
#[func]
pub fn measure(
    /// The engine.
    engine: &mut TypstEngine,
    /// The content whose size to measure.
    content: TypstContent,
    /// The styles with which to layout the content.
    styles: TypstStyles,
) -> TypstSourceResult<TypstDict> {
    let pod = TypstRegions::one(Axes::splat(TypstAbsLength::inf()), Axes::splat(false));
    let styles = TypstStyleChain::new(&styles);
    let frame = content.measure(engine, styles, pod)?.into_frame();
    let Size { x, y } = frame.size();
    Ok(dict! { "width" => x, "height" => y })
}
