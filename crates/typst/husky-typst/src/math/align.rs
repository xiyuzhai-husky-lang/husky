use crate::diag::SourceResult;
use crate::foundations::{elem, StyleChain, TypstContentRefined};
use crate::layout::TypstAbsLength;
use crate::math::{MathContext, MathFragment, MathRow, TypstLayoutMath};

/// A math alignment point: `&`, `&&`.
#[elem(title = "Alignment Point", TypstLayoutMath)]
pub struct TypstAlignPointElem {}

impl TypstLayoutMath for TypstContentRefined<TypstAlignPointElem> {
    fn layout_math(&self, ctx: &mut MathContext, _: StyleChain) -> SourceResult<()> {
        ctx.push(MathFragment::Align);
        Ok(())
    }
}

pub(super) struct TypstAlignmentResult {
    pub points: Vec<TypstAbsLength>,
    pub width: TypstAbsLength,
}

/// Determine the position of the alignment points.
pub(super) fn alignments(rows: &[MathRow]) -> TypstAlignmentResult {
    let mut widths = Vec::<TypstAbsLength>::new();

    let mut pending_width = TypstAbsLength::zero();
    for row in rows {
        let mut width = TypstAbsLength::zero();
        let mut alignment_index = 0;

        for fragment in row.iter() {
            if matches!(fragment, MathFragment::Align) {
                if alignment_index < widths.len() {
                    widths[alignment_index].set_max(width);
                } else {
                    widths.push(width.max(pending_width));
                }
                width = TypstAbsLength::zero();
                alignment_index += 1;
            } else {
                width += fragment.width();
            }
        }
        if widths.is_empty() {
            pending_width.set_max(width);
        } else if alignment_index < widths.len() {
            widths[alignment_index].set_max(width);
        } else {
            widths.push(width.max(pending_width));
        }
    }

    let mut points = widths;
    for i in 1..points.len() {
        let prev = points[i - 1];
        points[i] += prev;
    }
    TypstAlignmentResult {
        width: points.last().copied().unwrap_or(pending_width),
        points,
    }
}
