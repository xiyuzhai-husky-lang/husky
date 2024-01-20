pub mod group;
pub mod image;
pub mod mesh;
pub mod primitive;
pub mod rich_text;
pub mod shape;
pub mod text;
pub mod video;

use self::group::*;
use self::image::*;
use self::mesh::*;
use self::primitive::*;
use self::rich_text::*;
use self::shape::*;
use self::text::*;
use self::video::*;
use crate::synchrotron::VisualSynchrotron;
use serde::{Deserialize, Serialize};
use shifted_unsigned_int::ShiftedU32;

pub struct VisualId(ShiftedU32);

#[enum_class::from_variants]
pub enum Visual {
    Primitive(PrimitiveVisual),
    Text(TextVisual),
    RichText(RichTextVisual),
    Image(ImageVisual),
    Shape(ShapeVisual),
    Mesh(MeshVisual),
    Video(VideoVisual),
}

#[enum_class::from_variants]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VisualData {
    Text(TextVisualData),
    RichText(RichTextVisualData),
    Image(ImageVisualData),
    Shape(ShapeVisualData),
    Mesh(MeshVisualData),
    Video(VideoVisualData),
    Group(GroupVisualData),
}

impl VisualId {
    pub fn data<'a>(self, visual_synchrotron: &'a VisualSynchrotron) -> &'a VisualData {
        &visual_synchrotron.visual_arena()[self]
    }

    fn index(self) -> usize {
        self.0.into()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualArena(Vec<VisualData>);

impl std::ops::Index<VisualId> for VisualArena {
    type Output = VisualData;

    fn index(&self, id: VisualId) -> &Self::Output {
        &self.0[id.index()]
    }
}
