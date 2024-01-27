use super::*;
use husky_visual_protocol::visual::shape::Point;
use smallvec::{smallvec, SmallVec};

pub struct MnistSkeleton {
    strokes: MnistStrokes,
}

impl MnistSkeleton {
    pub(crate) fn one() -> Self {
        Self {
            strokes: smallvec![MnistStroke::LineSegment {
                start: Point::new(14.0, 1.0),
                end: Point::new(14.0, 27.0),
            }],
        }
    }
}

pub enum MnistStroke {
    LineSegment { start: Point, end: Point },
}

pub type MnistStrokes = SmallVec<[MnistStroke; 2]>;

impl Visualize for MnistSkeleton {
    fn visualize(&self, visual_synchrotron: &mut VisualSynchrotron) -> Visual {
        match self.strokes.len() {
            0 => todo!(),
            1 => self.strokes[0].visualize(visual_synchrotron),
            _ => Visual::new_group_visual(
                self.strokes
                    .iter()
                    .map(|stroke| stroke.visualize(visual_synchrotron))
                    .collect(),
                visual_synchrotron,
            ),
        }
    }
}

impl Visualize for MnistStroke {
    fn visualize(&self, visual_synchrotron: &mut VisualSynchrotron) -> Visual {
        match *self {
            MnistStroke::LineSegment { start, end } => {
                Visual::new_line_segment(start.into(), end.into(), visual_synchrotron)
            }
        }
    }
}
