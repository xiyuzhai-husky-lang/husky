use crate::diag::SourceResult;
use crate::foundations::{elem, StyleChain, TexContentRefined};
use crate::layout::TexAbsLength;
use crate::math::{MathContext, MathFragment, MathRow, TexLayoutMath};

/// A math alignment point: `&`, `&&`.
#[elem(title = "Alignment Point", TexLayoutMath)]
pub struct TexAlignPointElem {}

impl TexLayoutMath for TexContentRefined<TexAlignPointElem> {
    fn layout_math(&self, ctx: &mut MathContext, _: StyleChain) -> SourceResult<()> {
        ctx.push(MathFragment::Align);
        Ok(())
    }
}

pub(super) struct TexAlignmentResult {
    pub points: Vec<TexAbsLength>,
    pub width: TexAbsLength,
}

/// Determine the position of the alignment points.
pub(super) fn alignments(rows: &[MathRow]) -> TexAlignmentResult {
    let mut widths = Vec::<TexAbsLength>::new();

    let mut pending_width = TexAbsLength::zero();
    for row in rows {
        let mut width = TexAbsLength::zero();
        let mut alignment_index = 0;

        for fragment in row.iter() {
            if matches!(fragment, MathFragment::Align) {
                if alignment_index < widths.len() {
                    widths[alignment_index].set_max(width);
                } else {
                    widths.push(width.max(pending_width));
                }
                width = TexAbsLength::zero();
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
    TexAlignmentResult {
        width: points.last().copied().unwrap_or(pending_width),
        points,
    }
}
