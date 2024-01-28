use super::*;
use husky_visual_protocol::visual::shape::{Color, Point};
use smallvec::{smallvec, SmallVec};

pub struct MnistSkeleton {
    bones: MnistBones,
}

impl MnistSkeleton {
    pub(crate) fn one() -> Self {
        Self {
            bones: smallvec![MnistBone::LineSegment {
                start: Point::new(14.0, 1.0),
                end: Point::new(14.0, 27.0),
            }],
        }
    }

    pub fn bones(&self) -> &[MnistBone] {
        &self.bones
    }
}

pub enum MnistBone {
    LineSegment { start: Point, end: Point },
}

pub type MnistBones = SmallVec<[MnistBone; 2]>;

impl Visualize for MnistSkeleton {
    fn visualize(&self, visual_synchrotron: &mut VisualSynchrotron) -> Visual {
        match self.bones.len() {
            0 => todo!(),
            1 => self.bones[0].visualize(visual_synchrotron),
            _ => Visual::new_group_visual(
                self.bones
                    .iter()
                    .map(|stroke| stroke.visualize(visual_synchrotron))
                    .collect(),
                visual_synchrotron,
            ),
        }
    }
}

impl Visualize for MnistBone {
    fn visualize(&self, visual_synchrotron: &mut VisualSynchrotron) -> Visual {
        match *self {
            MnistBone::LineSegment { start, end } => Visual::new_line_segment(
                start.into(),
                end.into(),
                (2.0, Color::Yellow),
                visual_synchrotron,
            ),
        }
    }
}
