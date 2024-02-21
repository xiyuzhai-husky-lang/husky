use crate::diag::{bail, TypstSourceResult};
use crate::engine::TypstEngine;
use crate::foundations::{elem, TypstContentRefined, TypstStyleChain};
use crate::layout::{
    Angle, Axes, LayoutSingle, Rel, Size, TypstAbsLength, TypstFrame, TypstFrameItem, TypstLength,
    TypstRegions,
};
use crate::util::TypstNumeric;
use crate::visualize::{TypstGeometry, TypstStroke};

/// A line from one point to another.
///
/// # Example
/// ```example
/// #set page(height: 100pt)
///
/// #line(length: 100%)
/// #line(end: (50%, 50%))
/// #line(
///   length: 4cm,
///   stroke: 2pt + maroon,
/// )
/// ```
#[elem(LayoutSingle)]
pub struct LineElem {
    /// The start point of the line.
    ///
    /// Must be an array of exactly two relative lengths.
    #[resolve]
    pub start: Axes<Rel<TypstLength>>,

    /// The offset from `start` where the line ends.
    #[resolve]
    pub end: Option<Axes<Rel<TypstLength>>>,

    /// The line's length. This is only respected if `end` is `none`.
    #[resolve]
    #[default(TypstAbsLength::pt(30.0).into())]
    pub length: Rel<TypstLength>,

    /// The angle at which the line points away from the origin. This is only
    /// respected if `end` is `none`.
    pub angle: Angle,

    /// How to [stroke]($stroke) the line.
    ///
    /// ```example
    /// #set line(length: 100%)
    /// #stack(
    ///   spacing: 1em,
    ///   line(stroke: 2pt + red),
    ///   line(stroke: (paint: blue, thickness: 4pt, cap: "round")),
    ///   line(stroke: (paint: blue, thickness: 1pt, dash: "dashed")),
    ///   line(stroke: (paint: blue, thickness: 1pt, dash: ("dot", 2pt, 4pt, 2pt))),
    /// )
    /// ```
    #[resolve]
    #[fold]
    pub stroke: TypstStroke,
}

impl LayoutSingle for TypstContentRefined<LineElem> {
    #[husky_typst_macros::time(name = "line", span = self.span())]
    fn layout(
        &self,
        _: &mut TypstEngine,
        styles: TypstStyleChain,
        regions: TypstRegions,
    ) -> TypstSourceResult<TypstFrame> {
        let resolve =
            |axes: Axes<Rel<TypstAbsLength>>| axes.zip_map(regions.base(), Rel::relative_to);
        let start = resolve(self.start(styles));
        let delta = self
            .end(styles)
            .map(|end| resolve(end) - start)
            .unwrap_or_else(|| {
                let length = self.length(styles);
                let angle = self.angle(styles);
                let x = angle.cos() * length;
                let y = angle.sin() * length;
                resolve(Axes::new(x, y))
            });

        let stroke = self.stroke(styles).unwrap_or_default();
        let size = start.max(start + delta).max(Size::zero());
        let target = regions.expand.select(regions.size, size);

        if !target.is_finite() {
            bail!(self.span(), "cannot create line with infinite length");
        }

        let mut frame = TypstFrame::soft(target);
        let shape = TypstGeometry::Line(delta.to_point()).stroked(stroke);
        frame.push(start.to_point(), TypstFrameItem::Shape(shape, self.span()));
        Ok(frame)
    }
}
