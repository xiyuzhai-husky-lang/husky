use std::f64::consts::SQRT_2;

use crate::diag::TypstSourceResult;
use crate::engine::TypstEngine;
use crate::foundations::{
    elem, Resolve, Smart, TypstContent, TypstContentRefined, TypstStyleChain,
};
use crate::layout::{
    Axes, Corner, Corners, LayoutMultiple, Ratio, Rel, Sides, Size, TypstAbsLength, TypstFrame,
    TypstFrameItem, TypstLayoutSingle, TypstLength, TypstPoint, TypstRegions,
};
use crate::syntax::TypstSynSpan;
use crate::util::Get;
use crate::visualize::{Path, TypstFixedStroke, TypstPaint, TypstStroke};

/// A rectangle with optional content.
///
/// # Example
/// ```example
/// // Without content.
/// #rect(width: 35%, height: 30pt)
///
/// // With content.
/// #rect[
///   Automatically sized \
///   to fit the content.
/// ]
/// ```
#[elem(title = "Rectangle", TypstLayoutSingle)]
pub struct RectElem {
    /// The rectangle's width, relative to its parent container.
    pub width: Smart<Rel<TypstLength>>,

    /// The rectangle's height, relative to its parent container.
    pub height: Smart<Rel<TypstLength>>,

    /// How to fill the rectangle.
    ///
    /// When setting a fill, the default stroke disappears. To create a
    /// rectangle with both fill and stroke, you have to configure both.
    ///
    /// ```example
    /// #rect(fill: blue)
    /// ```
    pub fill: Option<TypstPaint>,

    /// How to stroke the rectangle. This can be:
    ///
    /// - `{none}` to disable stroking
    /// - `{auto}` for a stroke of `{1pt + black}` if and if only if no fill is
    ///   given.
    /// - Any kind of [stroke]($stroke)
    /// - A dictionary describing the stroke for each side inidvidually. The
    ///   dictionary can contain the following keys in order of precedence:
    ///   - `top`: The top stroke.
    ///   - `right`: The right stroke.
    ///   - `bottom`: The bottom stroke.
    ///   - `left`: The left stroke.
    ///   - `x`: The horizontal stroke.
    ///   - `y`: The vertical stroke.
    ///   - `rest`: The stroke on all sides except those for which the
    ///     dictionary explicitly sets a size.
    ///
    /// ```example
    /// #stack(
    ///   dir: ltr,
    ///   spacing: 1fr,
    ///   rect(stroke: red),
    ///   rect(stroke: 2pt),
    ///   rect(stroke: 2pt + red),
    /// )
    /// ```
    #[resolve]
    #[fold]
    pub stroke: Smart<Sides<Option<Option<TypstStroke>>>>,

    /// How much to round the rectangle's corners, relative to the minimum of
    /// the width and height divided by two. This can be:
    ///
    /// - A relative length for a uniform corner radius.
    /// - A dictionary: With a dictionary, the stroke for each side can be set
    ///   individually. The dictionary can contain the following keys in order
    ///   of precedence:
    ///   - `top-left`: The top-left corner radius.
    ///   - `top-right`: The top-right corner radius.
    ///   - `bottom-right`: The bottom-right corner radius.
    ///   - `bottom-left`: The bottom-left corner radius.
    ///   - `left`: The top-left and bottom-left corner radii.
    ///   - `top`: The top-left and top-right corner radii.
    ///   - `right`: The top-right and bottom-right corner radii.
    ///   - `bottom`: The bottom-left and bottom-right corner radii.
    ///   - `rest`: The radii for all corners except those for which the
    ///     dictionary explicitly sets a size.
    ///
    /// ```example
    /// #set rect(stroke: 4pt)
    /// #rect(
    ///   radius: (
    ///     left: 5pt,
    ///     top-right: 20pt,
    ///     bottom-right: 10pt,
    ///   ),
    ///   stroke: (
    ///     left: red,
    ///     top: yellow,
    ///     right: green,
    ///     bottom: blue,
    ///   ),
    /// )
    /// ```
    #[resolve]
    #[fold]
    pub radius: Corners<Option<Rel<TypstLength>>>,

    /// How much to pad the rectangle's content.
    /// See the [box's documentation]($box.outset) for more details.
    #[resolve]
    #[fold]
    #[default(Sides::splat(Some(TypstAbsLength::pt(5.0).into())))]
    pub inset: Sides<Option<Rel<TypstLength>>>,

    /// How much to expand the rectangle's size without affecting the layout.
    /// See the [box's documentation]($box.outset) for more details.
    #[resolve]
    #[fold]
    pub outset: Sides<Option<Rel<TypstLength>>>,

    /// The content to place into the rectangle.
    ///
    /// When this is omitted, the rectangle takes on a default size of at most
    /// `{45pt}` by `{30pt}`.
    #[positional]
    pub body: Option<TypstContent>,
}

impl TypstLayoutSingle for TypstContentRefined<RectElem> {
    #[husky_typst_macros::time(name = "rect", span = self.span())]
    fn layout(
        &self,
        engine: &mut TypstEngine,
        styles: TypstStyleChain,
        regions: TypstRegions,
    ) -> TypstSourceResult<TypstFrame> {
        layout(
            engine,
            styles,
            regions,
            ShapeKind::Rect,
            &self.body(styles),
            Axes::new(self.width(styles), self.height(styles)),
            self.fill(styles),
            self.stroke(styles),
            self.inset(styles),
            self.outset(styles),
            self.radius(styles),
            self.span(),
        )
    }
}

/// A square with optional content.
///
/// # Example
/// ```example
/// // Without content.
/// #square(size: 40pt)
///
/// // With content.
/// #square[
///   Automatically \
///   sized to fit.
/// ]
/// ```
#[elem(TypstLayoutSingle)]
pub struct SquareElem {
    /// The square's side length. This is mutually exclusive with `width` and
    /// `height`.
    #[external]
    pub size: Smart<TypstLength>,

    /// The square's width. This is mutually exclusive with `size` and `height`.
    ///
    /// In contrast to `size`, this can be relative to the parent container's
    /// width.
    #[parse(
        let size = args.named::<Smart<TypstLength>>("size")?.map(|s| s.map(Rel::from));
        match size {
            None => args.named("width")?,
            size => size,
        }
    )]
    pub width: Smart<Rel<TypstLength>>,

    /// The square's height. This is mutually exclusive with `size` and `width`.
    ///
    /// In contrast to `size`, this can be relative to the parent container's
    /// height.
    #[parse(match size {
        None => args.named("height")?,
        size => size,
    })]
    pub height: Smart<Rel<TypstLength>>,

    /// How to fill the square. See the [rectangle's documentation]($rect.fill)
    /// for more details.
    pub fill: Option<TypstPaint>,

    /// How to stroke the square. See the
    /// [rectangle's documentation]($rect.stroke) for more details.
    #[resolve]
    #[fold]
    pub stroke: Smart<Sides<Option<Option<TypstStroke>>>>,

    /// How much to round the square's corners. See the
    /// [rectangle's documentation]($rect.radius) for more details.
    #[resolve]
    #[fold]
    pub radius: Corners<Option<Rel<TypstLength>>>,

    /// How much to pad the square's content. See the
    /// [box's documentation]($box.inset) for more details.
    #[resolve]
    #[fold]
    #[default(Sides::splat(Some(TypstAbsLength::pt(5.0).into())))]
    pub inset: Sides<Option<Rel<TypstLength>>>,

    /// How much to expand the square's size without affecting the layout. See
    /// the [box's documentation]($box.outset) for more details.
    #[resolve]
    #[fold]
    pub outset: Sides<Option<Rel<TypstLength>>>,

    /// The content to place into the square. The square expands to fit this
    /// content, keeping the 1-1 aspect ratio.
    ///
    /// When this is omitted, the square takes on a default size of at most
    /// `{30pt}`.
    #[positional]
    pub body: Option<TypstContent>,
}

impl TypstLayoutSingle for TypstContentRefined<SquareElem> {
    #[husky_typst_macros::time(name = "square", span = self.span())]
    fn layout(
        &self,
        engine: &mut TypstEngine,
        styles: TypstStyleChain,
        regions: TypstRegions,
    ) -> TypstSourceResult<TypstFrame> {
        layout(
            engine,
            styles,
            regions,
            ShapeKind::Square,
            &self.body(styles),
            Axes::new(self.width(styles), self.height(styles)),
            self.fill(styles),
            self.stroke(styles),
            self.inset(styles),
            self.outset(styles),
            self.radius(styles),
            self.span(),
        )
    }
}

/// An ellipse with optional content.
///
/// # Example
/// ```example
/// // Without content.
/// #ellipse(width: 35%, height: 30pt)
///
/// // With content.
/// #ellipse[
///   #set align(center)
///   Automatically sized \
///   to fit the content.
/// ]
/// ```
#[elem(TypstLayoutSingle)]
pub struct EllipseElem {
    /// The ellipse's width, relative to its parent container.
    pub width: Smart<Rel<TypstLength>>,

    /// The ellipse's height, relative to its parent container.
    pub height: Smart<Rel<TypstLength>>,

    /// How to fill the ellipse. See the [rectangle's documentation]($rect.fill)
    /// for more details.
    pub fill: Option<TypstPaint>,

    /// How to stroke the ellipse. See the
    /// [rectangle's documentation]($rect.stroke) for more details.
    #[resolve]
    #[fold]
    pub stroke: Smart<Option<TypstStroke>>,

    /// How much to pad the ellipse's content. See the
    /// [box's documentation]($box.inset) for more details.
    #[resolve]
    #[fold]
    #[default(Sides::splat(Some(TypstAbsLength::pt(5.0).into())))]
    pub inset: Sides<Option<Rel<TypstLength>>>,

    /// How much to expand the ellipse's size without affecting the layout. See
    /// the [box's documentation]($box.outset) for more details.
    #[resolve]
    #[fold]
    pub outset: Sides<Option<Rel<TypstLength>>>,

    /// The content to place into the ellipse.
    ///
    /// When this is omitted, the ellipse takes on a default size of at most
    /// `{45pt}` by `{30pt}`.
    #[positional]
    pub body: Option<TypstContent>,
}

impl TypstLayoutSingle for TypstContentRefined<EllipseElem> {
    #[husky_typst_macros::time(name = "ellipse", span = self.span())]
    fn layout(
        &self,
        engine: &mut TypstEngine,
        styles: TypstStyleChain,
        regions: TypstRegions,
    ) -> TypstSourceResult<TypstFrame> {
        layout(
            engine,
            styles,
            regions,
            ShapeKind::Ellipse,
            &self.body(styles),
            Axes::new(self.width(styles), self.height(styles)),
            self.fill(styles),
            self.stroke(styles).map(|s| Sides::splat(Some(s))),
            self.inset(styles),
            self.outset(styles),
            Corners::splat(None),
            self.span(),
        )
    }
}

/// A circle with optional content.
///
/// # Example
/// ```example
/// // Without content.
/// #circle(radius: 25pt)
///
/// // With content.
/// #circle[
///   #set align(center + horizon)
///   Automatically \
///   sized to fit.
/// ]
/// ```
#[elem(TypstLayoutSingle)]
pub struct CircleElem {
    /// The circle's radius. This is mutually exclusive with `width` and
    /// `height`.
    #[external]
    pub radius: TypstLength,

    /// The circle's width. This is mutually exclusive with `radius` and
    /// `height`.
    ///
    /// In contrast to `radius`, this can be relative to the parent container's
    /// width.
    #[parse(
        let size = args
            .named::<Smart<TypstLength>>("radius")?
            .map(|s| s.map(|r| 2.0 * Rel::from(r)));
        match size {
            None => args.named("width")?,
            size => size,
        }
    )]
    pub width: Smart<Rel<TypstLength>>,

    /// The circle's height. This is mutually exclusive with `radius` and
    /// `width`.
    ///
    /// In contrast to `radius`, this can be relative to the parent container's
    /// height.
    #[parse(match size {
        None => args.named("height")?,
        size => size,
    })]
    pub height: Smart<Rel<TypstLength>>,

    /// How to fill the circle. See the [rectangle's documentation]($rect.fill)
    /// for more details.
    pub fill: Option<TypstPaint>,

    /// How to stroke the circle. See the
    /// [rectangle's documentation]($rect.stroke) for more details.
    #[resolve]
    #[fold]
    #[default(Smart::Auto)]
    pub stroke: Smart<Option<TypstStroke>>,

    /// How much to pad the circle's content. See the
    /// [box's documentation]($box.inset) for more details.
    #[resolve]
    #[fold]
    #[default(Sides::splat(Some(TypstAbsLength::pt(5.0).into())))]
    pub inset: Sides<Option<Rel<TypstLength>>>,

    /// How much to expand the circle's size without affecting the layout. See
    /// the [box's documentation]($box.outset) for more details.
    #[resolve]
    #[fold]
    pub outset: Sides<Option<Rel<TypstLength>>>,

    /// The content to place into the circle. The circle expands to fit this
    /// content, keeping the 1-1 aspect ratio.
    #[positional]
    pub body: Option<TypstContent>,
}

impl TypstLayoutSingle for TypstContentRefined<CircleElem> {
    #[husky_typst_macros::time(name = "circle", span = self.span())]
    fn layout(
        &self,
        engine: &mut TypstEngine,
        styles: TypstStyleChain,
        regions: TypstRegions,
    ) -> TypstSourceResult<TypstFrame> {
        layout(
            engine,
            styles,
            regions,
            ShapeKind::Circle,
            &self.body(styles),
            Axes::new(self.width(styles), self.height(styles)),
            self.fill(styles),
            self.stroke(styles).map(|s| Sides::splat(Some(s))),
            self.inset(styles),
            self.outset(styles),
            Corners::splat(None),
            self.span(),
        )
    }
}

/// Layout a shape.
#[allow(clippy::too_many_arguments)]
fn layout(
    engine: &mut TypstEngine,
    styles: TypstStyleChain,
    regions: TypstRegions,
    kind: ShapeKind,
    body: &Option<TypstContent>,
    sizing: Axes<Smart<Rel<TypstLength>>>,
    fill: Option<TypstPaint>,
    stroke: Smart<Sides<Option<Option<TypstStroke<TypstAbsLength>>>>>,
    inset: Sides<Option<Rel<TypstAbsLength>>>,
    outset: Sides<Option<Rel<TypstAbsLength>>>,
    radius: Corners<Option<Rel<TypstAbsLength>>>,
    span: TypstSynSpan,
) -> TypstSourceResult<TypstFrame> {
    let resolved = sizing.zip_map(regions.base(), |s, r| {
        s.map(|v| v.resolve(styles).relative_to(r))
    });

    let mut frame;
    let mut inset = inset.unwrap_or_default();

    if let Some(child) = body {
        let region = resolved.unwrap_or(regions.base());

        if kind.is_round() {
            inset = inset.map(|side| side + Ratio::new(0.5 - SQRT_2 / 4.0));
        }

        // Pad the child.
        let child = child
            .clone()
            .padded(inset.map(|side| side.map(TypstLength::from)));
        let expand = sizing.as_ref().map(Smart::is_custom);
        let pod = TypstRegions::one(region, expand);
        frame = child.layout(engine, styles, pod)?.into_frame();

        // Enforce correct size.
        *frame.size_mut() = expand.select(region, frame.size());

        // Relayout with full expansion into square region to make sure
        // the result is really a square or circle.
        if kind.is_quadratic() {
            frame.set_size(Size::splat(frame.size().max_by_side()));
            let length = frame.size().max_by_side().min(region.min_by_side());
            let pod = TypstRegions::one(Size::splat(length), Axes::splat(true));
            frame = child.layout(engine, styles, pod)?.into_frame();
        }

        // Enforce correct size again.
        *frame.size_mut() = expand.select(region, frame.size());
        if kind.is_quadratic() {
            frame.set_size(Size::splat(frame.size().max_by_side()));
        }
    } else {
        // The default size that a shape takes on if it has no child and
        // enough space.
        let default = Size::new(TypstAbsLength::pt(45.0), TypstAbsLength::pt(30.0));
        let mut size = resolved.unwrap_or(default.min(regions.base()));
        if kind.is_quadratic() {
            size = Size::splat(size.min_by_side());
        }
        frame = TypstFrame::soft(size);
    }

    // Prepare stroke.
    let stroke = match stroke {
        Smart::Auto if fill.is_none() => Sides::splat(Some(TypstFixedStroke::default())),
        Smart::Auto => Sides::splat(None),
        Smart::Custom(strokes) => strokes
            .unwrap_or_default()
            .map(|s| s.map(TypstStroke::unwrap_or_default)),
    };

    // Add fill and/or stroke.
    if fill.is_some() || stroke.iter().any(Option::is_some) {
        if kind.is_round() {
            let outset = outset.unwrap_or_default().relative_to(frame.size());
            let size = frame.size() + outset.sum_by_axis();
            let pos = TypstPoint::new(-outset.left, -outset.top);
            let shape = ellipse(size, fill, stroke.left);
            frame.prepend(pos, TypstFrameItem::Shape(shape, span));
        } else {
            frame.fill_and_stroke(
                fill,
                stroke,
                outset.unwrap_or_default(),
                radius.unwrap_or_default(),
                span,
            );
        }
    }

    Ok(frame)
}

/// A category of shape.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ShapeKind {
    /// A rectangle with equal side lengths.
    Square,
    /// A quadrilateral with four right angles.
    Rect,
    /// An ellipse with coinciding foci.
    Circle,
    /// A curve around two focal points.
    Ellipse,
}

impl ShapeKind {
    /// Whether this shape kind is curvy.
    fn is_round(self) -> bool {
        matches!(self, Self::Circle | Self::Ellipse)
    }

    /// Whether this shape kind has equal side length.
    fn is_quadratic(self) -> bool {
        matches!(self, Self::Square | Self::Circle)
    }
}

/// A geometric shape with optional fill and stroke.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct TypstShape {
    /// The shape's geometry.
    pub geometry: TypstGeometry,
    /// The shape's background fill.
    pub fill: Option<TypstPaint>,
    /// The shape's border stroke.
    pub stroke: Option<TypstFixedStroke>,
}

/// A shape's geometry.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum TypstGeometry {
    /// A line to a point (relative to its position).
    Line(TypstPoint),
    /// A rectangle with its origin in the topleft corner.
    Rect(Size),
    /// A bezier path.
    Path(Path),
}

impl TypstGeometry {
    /// Fill the geometry without a stroke.
    pub fn filled(self, fill: TypstPaint) -> TypstShape {
        TypstShape {
            geometry: self,
            fill: Some(fill),
            stroke: None,
        }
    }

    /// Stroke the geometry without a fill.
    pub fn stroked(self, stroke: TypstFixedStroke) -> TypstShape {
        TypstShape {
            geometry: self,
            fill: None,
            stroke: Some(stroke),
        }
    }

    /// The bounding box of the geometry.
    pub fn bbox_size(&self) -> Size {
        match self {
            Self::Line(line) => Size::new(line.x, line.y),
            Self::Rect(s) => *s,
            Self::Path(p) => p.bbox_size(),
        }
    }
}

/// Produce a shape that approximates an axis-aligned ellipse.
pub(crate) fn ellipse(
    size: Size,
    fill: Option<TypstPaint>,
    stroke: Option<TypstFixedStroke>,
) -> TypstShape {
    // https://stackoverflow.com/a/2007782
    let z = TypstAbsLength::zero();
    let rx = size.x / 2.0;
    let ry = size.y / 2.0;
    let m = 0.551784;
    let mx = m * rx;
    let my = m * ry;
    let point = |x, y| TypstPoint::new(x + rx, y + ry);

    let mut path = Path::new();
    path.move_to(point(-rx, z));
    path.cubic_to(point(-rx, -my), point(-mx, -ry), point(z, -ry));
    path.cubic_to(point(mx, -ry), point(rx, -my), point(rx, z));
    path.cubic_to(point(rx, my), point(mx, ry), point(z, ry));
    path.cubic_to(point(-mx, ry), point(-rx, my), point(-rx, z));

    TypstShape {
        geometry: TypstGeometry::Path(path),
        stroke,
        fill,
    }
}

/// Creates a new rectangle as a path.
pub(crate) fn clip_rect(
    size: Size,
    radius: Corners<Rel<TypstAbsLength>>,
    stroke: &Sides<Option<TypstFixedStroke>>,
) -> Path {
    let stroke_widths = stroke.as_ref().map(|s| {
        s.as_ref()
            .map_or(TypstAbsLength::zero(), |s| s.thickness / 2.0)
    });

    let max_radius = (size.x.min(size.y)) / 2.0
        + stroke_widths
            .iter()
            .cloned()
            .min()
            .unwrap_or(TypstAbsLength::zero());

    let radius = radius.map(|side| side.relative_to(max_radius * 2.0).min(max_radius));

    let corners = corners_control_points(size, radius, stroke, stroke_widths);

    let mut path = Path::new();
    if corners.top_left.arc_inner() {
        path.arc_move(
            corners.top_left.start_inner(),
            corners.top_left.center_inner(),
            corners.top_left.end_inner(),
        );
    } else {
        path.move_to(corners.top_left.center_inner());
    }
    for corner in [
        &corners.top_right,
        &corners.bottom_right,
        &corners.bottom_left,
    ] {
        if corner.arc_inner() {
            path.arc_line(
                corner.start_inner(),
                corner.center_inner(),
                corner.end_inner(),
            )
        } else {
            path.line_to(corner.center_inner());
        }
    }
    path.close_path();
    path
}

/// Create a styled rectangle with shapes.
/// - use rect primitive for simple rectangles
/// - stroke sides if possible
/// - use fill for sides for best looks
pub(crate) fn styled_rect(
    size: Size,
    radius: Corners<Rel<TypstAbsLength>>,
    fill: Option<TypstPaint>,
    stroke: Sides<Option<TypstFixedStroke>>,
) -> Vec<TypstShape> {
    if stroke.is_uniform() && radius.iter().cloned().all(Rel::is_zero) {
        simple_rect(size, fill, stroke.top)
    } else {
        segmented_rect(size, radius, fill, stroke)
    }
}

/// Use rect primitive for the rectangle
fn simple_rect(
    size: Size,
    fill: Option<TypstPaint>,
    stroke: Option<TypstFixedStroke>,
) -> Vec<TypstShape> {
    vec![TypstShape {
        geometry: TypstGeometry::Rect(size),
        fill,
        stroke,
    }]
}

fn corners_control_points(
    size: Size,
    radius: Corners<TypstAbsLength>,
    strokes: &Sides<Option<TypstFixedStroke>>,
    stroke_widths: Sides<TypstAbsLength>,
) -> Corners<ControlPoints> {
    Corners {
        top_left: Corner::TopLeft,
        top_right: Corner::TopRight,
        bottom_right: Corner::BottomRight,
        bottom_left: Corner::BottomLeft,
    }
    .map(|corner| ControlPoints {
        radius: radius.get(corner),
        stroke_before: stroke_widths.get(corner.side_ccw()),
        stroke_after: stroke_widths.get(corner.side_cw()),
        corner,
        size,
        same: match (
            strokes.get_ref(corner.side_ccw()),
            strokes.get_ref(corner.side_cw()),
        ) {
            (Some(a), Some(b)) => a.paint == b.paint && a.dash == b.dash,
            (None, None) => true,
            _ => false,
        },
    })
}

/// Use stroke and fill for the rectangle
fn segmented_rect(
    size: Size,
    radius: Corners<Rel<TypstAbsLength>>,
    fill: Option<TypstPaint>,
    strokes: Sides<Option<TypstFixedStroke>>,
) -> Vec<TypstShape> {
    let mut res = vec![];
    let stroke_widths = strokes.as_ref().map(|s| {
        s.as_ref()
            .map_or(TypstAbsLength::zero(), |s| s.thickness / 2.0)
    });

    let max_radius = (size.x.min(size.y)) / 2.0
        + stroke_widths
            .iter()
            .cloned()
            .min()
            .unwrap_or(TypstAbsLength::zero());

    let radius = radius.map(|side| side.relative_to(max_radius * 2.0).min(max_radius));

    let corners = corners_control_points(size, radius, &strokes, stroke_widths);

    // insert stroked sides below filled sides
    let mut stroke_insert = 0;

    // fill shape with inner curve
    if let Some(fill) = fill {
        let mut path = Path::new();
        let c = corners.get_ref(Corner::TopLeft);
        if c.arc() {
            path.arc_move(c.start(), c.center(), c.end());
        } else {
            path.move_to(c.center());
        };

        for corner in [Corner::TopRight, Corner::BottomRight, Corner::BottomLeft] {
            let c = corners.get_ref(corner);
            if c.arc() {
                path.arc_line(c.start(), c.center(), c.end());
            } else {
                path.line_to(c.center());
            }
        }
        path.close_path();
        res.push(TypstShape {
            geometry: TypstGeometry::Path(path),
            fill: Some(fill),
            stroke: None,
        });
        stroke_insert += 1;
    }

    let current = corners.iter().find(|c| !c.same).map(|c| c.corner);
    if let Some(mut current) = current {
        // multiple segments
        // start at a corner with a change between sides and iterate clockwise all other corners
        let mut last = current;
        for _ in 0..4 {
            current = current.next_cw();
            if corners.get_ref(current).same {
                continue;
            }
            // create segment
            let start = last;
            let end = current;
            last = current;
            let stroke = match strokes.get_ref(start.side_cw()) {
                None => continue,
                Some(stroke) => stroke.clone(),
            };
            let (shape, ontop) = segment(start, end, &corners, stroke);
            if ontop {
                res.push(shape);
            } else {
                res.insert(stroke_insert, shape);
                stroke_insert += 1;
            }
        }
    } else if let Some(stroke) = strokes.top {
        // single segment
        let (shape, _) = segment(Corner::TopLeft, Corner::TopLeft, &corners, stroke);
        res.push(shape);
    }
    res
}

fn path_segment(start: Corner, end: Corner, corners: &Corners<ControlPoints>, path: &mut Path) {
    // create start corner
    let c = corners.get_ref(start);
    if start == end || !c.arc() {
        path.move_to(c.end());
    } else {
        path.arc_move(c.mid(), c.center(), c.end());
    }

    // create corners between start and end
    let mut current = start.next_cw();
    while current != end {
        let c = corners.get_ref(current);
        if c.arc() {
            path.arc_line(c.start(), c.center(), c.end());
        } else {
            path.line_to(c.end());
        }
        current = current.next_cw();
    }

    // create end corner
    let c = corners.get_ref(end);
    if !c.arc() {
        path.line_to(c.start());
    } else if start == end {
        path.arc_line(c.start(), c.center(), c.end());
    } else {
        path.arc_line(c.start(), c.center(), c.mid());
    }
}

/// Returns the shape for the segment and whether the shape should be drawn on top.
fn segment(
    start: Corner,
    end: Corner,
    corners: &Corners<ControlPoints>,
    stroke: TypstFixedStroke,
) -> (TypstShape, bool) {
    fn fill_corner(corner: &ControlPoints) -> bool {
        corner.stroke_before != corner.stroke_after || corner.radius() < corner.stroke_before
    }

    fn fill_corners(start: Corner, end: Corner, corners: &Corners<ControlPoints>) -> bool {
        if fill_corner(corners.get_ref(start)) {
            return true;
        }
        if fill_corner(corners.get_ref(end)) {
            return true;
        }
        let mut current = start.next_cw();
        while current != end {
            if fill_corner(corners.get_ref(current)) {
                return true;
            }
            current = current.next_cw();
        }
        false
    }

    let solid = stroke
        .dash
        .as_ref()
        .map(|pattern| pattern.array.is_empty())
        .unwrap_or(true);

    let use_fill = solid && fill_corners(start, end, corners);

    let shape = if use_fill {
        fill_segment(start, end, corners, stroke)
    } else {
        stroke_segment(start, end, corners, stroke)
    };
    (shape, use_fill)
}

/// Stroke the sides from `start` to `end` clockwise.
fn stroke_segment(
    start: Corner,
    end: Corner,
    corners: &Corners<ControlPoints>,
    stroke: TypstFixedStroke,
) -> TypstShape {
    // create start corner
    let mut path = Path::new();
    path_segment(start, end, corners, &mut path);

    TypstShape {
        geometry: TypstGeometry::Path(path),
        stroke: Some(stroke),
        fill: None,
    }
}

/// Fill the sides from `start` to `end` clockwise.
fn fill_segment(
    start: Corner,
    end: Corner,
    corners: &Corners<ControlPoints>,
    stroke: TypstFixedStroke,
) -> TypstShape {
    let mut path = Path::new();

    // create the start corner
    // begin on the inside and finish on the outside
    // no corner if start and end are equal
    // half corner if different
    if start == end {
        let c = corners.get_ref(start);
        path.move_to(c.end_inner());
        path.line_to(c.end_outer());
    } else {
        let c = corners.get_ref(start);

        if c.arc_inner() {
            path.arc_move(c.end_inner(), c.center_inner(), c.mid_inner());
        } else {
            path.move_to(c.end_inner());
        }

        if c.arc_outer() {
            path.arc_line(c.mid_outer(), c.center_outer(), c.end_outer());
        } else {
            path.line_to(c.outer());
            path.line_to(c.end_outer());
        }
    }

    // create the clockwise outside path for the corners between start and end
    let mut current = start.next_cw();
    while current != end {
        let c = corners.get_ref(current);
        if c.arc_outer() {
            path.arc_line(c.start_outer(), c.center_outer(), c.end_outer());
        } else {
            path.line_to(c.outer());
        }
        current = current.next_cw();
    }

    // create the end corner
    // begin on the outside and finish on the inside
    // full corner if start and end are equal
    // half corner if different
    if start == end {
        let c = corners.get_ref(end);
        if c.arc_outer() {
            path.arc_line(c.start_outer(), c.center_outer(), c.end_outer());
        } else {
            path.line_to(c.outer());
            path.line_to(c.end_outer());
        }
        if c.arc_inner() {
            path.arc_line(c.end_inner(), c.center_inner(), c.start_inner());
        } else {
            path.line_to(c.center_inner());
        }
    } else {
        let c = corners.get_ref(end);
        if c.arc_outer() {
            path.arc_line(c.start_outer(), c.center_outer(), c.mid_outer());
        } else {
            path.line_to(c.outer());
        }
        if c.arc_inner() {
            path.arc_line(c.mid_inner(), c.center_inner(), c.start_inner());
        } else {
            path.line_to(c.center_inner());
        }
    }

    // create the counterclockwise inside path for the corners between start and end
    let mut current = end.next_ccw();
    while current != start {
        let c = corners.get_ref(current);
        if c.arc_inner() {
            path.arc_line(c.end_inner(), c.center_inner(), c.start_inner());
        } else {
            path.line_to(c.center_inner());
        }
        current = current.next_ccw();
    }

    path.close_path();

    TypstShape {
        geometry: TypstGeometry::Path(path),
        stroke: None,
        fill: Some(stroke.paint),
    }
}

/// Helper to calculate different control points for the corners.
/// Clockwise orientation from start to end.
/// ```text
/// O-------------------EO  ---   - Z: Zero/Origin ({x: 0, y: 0} for top left corner)
/// |\   ___----'''     |    |    - O: Outer: intersection between the straight outer lines
/// | \ /               |    |    - S_: start
/// |  MO               |    |    - M_: midpoint
/// | /Z\  __-----------E    |    - E_: end
/// |/   \M             |    ro   - r_: radius
/// |    /\             |    |    - middle of the stroke
/// |   /  \            |    |      - arc from S through M to E with center C and radius r
/// |  |    MI--EI-------    |    - outer curve
/// |  |  /  \               |      - arc from SO through MO to EO with center CO and radius ro
/// SO | |    \         CO  ---   - inner curve
/// |  | |     \                    - arc from SI through MI to EI with center CI and radius ri
/// |--S-SI-----CI      C
///      |--ri--|
///    |-------r--------|
/// ```
struct ControlPoints {
    radius: TypstAbsLength,
    stroke_after: TypstAbsLength,
    stroke_before: TypstAbsLength,
    corner: Corner,
    size: Size,
    same: bool,
}

impl ControlPoints {
    /// Move and rotate the point from top-left to the required corner.
    fn rotate(&self, point: TypstPoint) -> TypstPoint {
        match self.corner {
            Corner::TopLeft => point,
            Corner::TopRight => TypstPoint {
                x: self.size.x - point.y,
                y: point.x,
            },
            Corner::BottomRight => TypstPoint {
                x: self.size.x - point.x,
                y: self.size.y - point.y,
            },
            Corner::BottomLeft => TypstPoint {
                x: point.y,
                y: self.size.y - point.x,
            },
        }
    }

    /// Outside intersection of the sides.
    pub fn outer(&self) -> TypstPoint {
        self.rotate(TypstPoint {
            x: -self.stroke_before,
            y: -self.stroke_after,
        })
    }

    /// Center for the outer arc.
    pub fn center_outer(&self) -> TypstPoint {
        let r = self.radius_outer();
        self.rotate(TypstPoint {
            x: r - self.stroke_before,
            y: r - self.stroke_after,
        })
    }

    /// Center for the middle arc.
    pub fn center(&self) -> TypstPoint {
        let r = self.radius();
        self.rotate(TypstPoint { x: r, y: r })
    }

    /// Center for the inner arc.
    pub fn center_inner(&self) -> TypstPoint {
        let r = self.radius_inner();

        self.rotate(TypstPoint {
            x: self.stroke_before + r,
            y: self.stroke_after + r,
        })
    }

    /// Radius of the outer arc.
    pub fn radius_outer(&self) -> TypstAbsLength {
        self.radius
    }

    /// Radius of the middle arc.
    pub fn radius(&self) -> TypstAbsLength {
        (self.radius - self.stroke_before.min(self.stroke_after)).max(TypstAbsLength::zero())
    }

    /// Radius of the inner arc.
    pub fn radius_inner(&self) -> TypstAbsLength {
        (self.radius - 2.0 * self.stroke_before.max(self.stroke_after)).max(TypstAbsLength::zero())
    }

    /// Middle of the corner on the outside of the stroke.
    pub fn mid_outer(&self) -> TypstPoint {
        let c_i = self.center_inner();
        let c_o = self.center_outer();
        let o = self.outer();
        let r = self.radius_outer();

        // https://math.stackexchange.com/a/311956
        // intersection between the line from inner center to outside and the outer arc
        let a = (o.x - c_i.x).to_raw().powi(2) + (o.y - c_i.y).to_raw().powi(2);
        let b = 2.0 * (o.x - c_i.x).to_raw() * (c_i.x - c_o.x).to_raw()
            + 2.0 * (o.y - c_i.y).to_raw() * (c_i.y - c_o.y).to_raw();
        let c = (c_i.x - c_o.x).to_raw().powi(2) + (c_i.y - c_o.y).to_raw().powi(2)
            - r.to_raw().powi(2);
        let t = (-b + (b * b - 4.0 * a * c).sqrt()) / (2.0 * a);
        c_i + t * (o - c_i)
    }

    /// Middle of the corner in the middle of the stroke.
    pub fn mid(&self) -> TypstPoint {
        let center = self.center_outer();
        let outer = self.outer();
        let diff = outer - center;
        center + diff / diff.hypot().to_raw() * self.radius().to_raw()
    }

    /// Middle of the corner on the inside of the stroke.
    pub fn mid_inner(&self) -> TypstPoint {
        let center = self.center_inner();
        let outer = self.outer();
        let diff = outer - center;
        center + diff / diff.hypot().to_raw() * self.radius_inner().to_raw()
    }

    /// If an outer arc is required.
    pub fn arc_outer(&self) -> bool {
        self.radius_outer() > TypstAbsLength::zero()
    }

    pub fn arc(&self) -> bool {
        self.radius() > TypstAbsLength::zero()
    }

    /// If an inner arc is required.
    pub fn arc_inner(&self) -> bool {
        self.radius_inner() > TypstAbsLength::zero()
    }

    /// Start of the corner on the outside of the stroke.
    pub fn start_outer(&self) -> TypstPoint {
        self.rotate(TypstPoint {
            x: -self.stroke_before,
            y: self.radius_outer() - self.stroke_after,
        })
    }

    /// Start of the corner in the center of the stroke.
    pub fn start(&self) -> TypstPoint {
        self.rotate(TypstPoint::with_y(self.radius()))
    }

    /// Start of the corner on the inside of the stroke.
    pub fn start_inner(&self) -> TypstPoint {
        self.rotate(TypstPoint {
            x: self.stroke_before,
            y: self.stroke_after + self.radius_inner(),
        })
    }

    /// End of the corner on the outside of the stroke.
    pub fn end_outer(&self) -> TypstPoint {
        self.rotate(TypstPoint {
            x: self.radius_outer() - self.stroke_before,
            y: -self.stroke_after,
        })
    }

    /// End of the corner in the center of the stroke.
    pub fn end(&self) -> TypstPoint {
        self.rotate(TypstPoint::with_x(self.radius()))
    }

    /// End of the corner on the inside of the stroke.
    pub fn end_inner(&self) -> TypstPoint {
        self.rotate(TypstPoint {
            x: self.stroke_before + self.radius_inner(),
            y: self.stroke_after,
        })
    }
}

/// Helper to draw arcs with bezier curves.
trait PathExt {
    fn arc(&mut self, start: TypstPoint, center: TypstPoint, end: TypstPoint);
    fn arc_move(&mut self, start: TypstPoint, center: TypstPoint, end: TypstPoint);
    fn arc_line(&mut self, start: TypstPoint, center: TypstPoint, end: TypstPoint);
}

impl PathExt for Path {
    fn arc(&mut self, start: TypstPoint, center: TypstPoint, end: TypstPoint) {
        let arc = bezier_arc_control(start, center, end);
        self.cubic_to(arc[0], arc[1], end);
    }

    fn arc_move(&mut self, start: TypstPoint, center: TypstPoint, end: TypstPoint) {
        self.move_to(start);
        self.arc(start, center, end);
    }

    fn arc_line(&mut self, start: TypstPoint, center: TypstPoint, end: TypstPoint) {
        self.line_to(start);
        self.arc(start, center, end);
    }
}

/// Get the control points for a bezier curve that approximates a circular arc for
/// a start point, an end point and a center of the circle whose arc connects
/// the two.
fn bezier_arc_control(start: TypstPoint, center: TypstPoint, end: TypstPoint) -> [TypstPoint; 2] {
    // https://stackoverflow.com/a/44829356/1567835
    let a = start - center;
    let b = end - center;

    let q1 = a.x.to_raw() * a.x.to_raw() + a.y.to_raw() * a.y.to_raw();
    let q2 = q1 + a.x.to_raw() * b.x.to_raw() + a.y.to_raw() * b.y.to_raw();
    let k2 = (4.0 / 3.0) * ((2.0 * q1 * q2).sqrt() - q2)
        / (a.x.to_raw() * b.y.to_raw() - a.y.to_raw() * b.x.to_raw());

    let control_1 = TypstPoint::new(center.x + a.x - k2 * a.y, center.y + a.y + k2 * a.x);
    let control_2 = TypstPoint::new(center.x + b.x + k2 * b.y, center.y + b.y - k2 * b.x);

    [control_1, control_2]
}
