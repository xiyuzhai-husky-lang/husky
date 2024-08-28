pub mod group;
pub mod image;
pub mod math;
pub mod mesh;
pub mod primitive;
pub mod rich_text;
pub mod shape;
pub mod text;
pub mod video;

use self::{
    group::*, image::*, math::*, mesh::*, primitive::*, rich_text::*, shape::*, text::*, video::*,
};
use crate::synchrotron::VisualSynchrotron;
use serde::{Deserialize, Serialize};
use shifted_unsigned_int::ShiftedU32;

#[enum_class::from_variants]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum Visual {
    Void,
    Image(ImageVisual),
    Math(MathVisual),
    Mesh(MeshVisual),
    Primitive(PrimitiveVisual),
    RichText(RichTextVisual),
    Shape(ShapeVisual),
    Text(TextVisual),
    Video(VideoVisual),
    // composites
    Group(GroupVisual),
}

#[enum_class::from_variants]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VisualData {
    Text(TextVisualData),
    RichText(RichTextVisualData),
    Image(ImageVisualData),
    Shape(ShapeVisualData),
    Math(MathVisualData),
    Mesh(MeshVisualData),
    Video(VideoVisualData),
    Group(GroupVisualData),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize, Hash)]
#[serde(from = "VisualSerdeId", into = "VisualSerdeId")]
pub struct VisualId(ShiftedU32);

impl From<VisualSerdeId> for VisualId {
    fn from(value: VisualSerdeId) -> Self {
        Self(value.0.into())
    }
}

impl Into<VisualSerdeId> for VisualId {
    fn into(self) -> VisualSerdeId {
        VisualSerdeId(self.0.into())
    }
}

macro_rules! impl_visual_serde_id_from_to_for_sub_visual_id {
    ($ty: ty) => {
        impl From<VisualSerdeId> for $ty {
            fn from(value: VisualSerdeId) -> Self {
                Self(value.into())
            }
        }

        impl Into<VisualSerdeId> for $ty {
            fn into(self) -> VisualSerdeId {
                self.0.into()
            }
        }
    };
}
use impl_visual_serde_id_from_to_for_sub_visual_id;

/// a vehicle for compactly serialize and deserialize VisualIds
///
/// shouldn't be accessible from outer crates
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize, Hash)]
#[serde(from = "usize", into = "usize")]
struct VisualSerdeId(usize);

impl From<usize> for VisualSerdeId {
    fn from(value: usize) -> Self {
        Self(value.into())
    }
}

impl Into<usize> for VisualSerdeId {
    fn into(self) -> usize {
        self.0.into()
    }
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

impl VisualArena {
    pub(crate) fn alloc(&mut self, data: VisualData) -> VisualId {
        let id = VisualId(self.0.len().into());
        self.0.push(data);
        id
    }
}

impl std::ops::Index<VisualId> for VisualArena {
    type Output = VisualData;

    fn index(&self, id: VisualId) -> &Self::Output {
        &self.0[id.index()]
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct CompositeVisual<Id> {
    pub followed_reduced: Option<(Id, Visual)>,
    pub accompanyings_except_followed_reduced: Vec<(Id, Visual)>,
}

impl<Id> std::ops::Index<Id> for CompositeVisual<Id>
where
    Id: Eq,
{
    type Output = Visual;

    fn index(&self, index: Id) -> &Self::Output {
        // Check if the index matches the followed_reduced option
        if let Some((id, visual)) = &self.followed_reduced {
            if *id == index {
                return visual;
            }
        }
        // Search through the accompanyings_except_followed_reduced vector
        for (id, visual) in &self.accompanyings_except_followed_reduced {
            if *id == index {
                return visual;
            }
        }
        // If no match is found, panic as per the contract of the Index trait
        panic!("No visual found for the given index");
    }
}
