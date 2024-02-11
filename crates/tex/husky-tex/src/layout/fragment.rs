use std::fmt::{self, Debug, Formatter};

use crate::layout::TexFrame;

/// A partial layout result.
#[derive(Clone)]
pub struct TexLayoutFragment(Vec<TexFrame>);

impl TexLayoutFragment {
    /// Create a fragment from a single frame.
    pub fn frame(frame: TexFrame) -> Self {
        Self(vec![frame])
    }

    /// Create a fragment from multiple frames.
    pub fn frames(frames: Vec<TexFrame>) -> Self {
        Self(frames)
    }

    /// Return `true` if the length is 0.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// The number of frames in the fragment.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Extract the first and only frame.
    ///
    /// Panics if there are multiple frames.
    #[track_caller]
    pub fn into_frame(self) -> TexFrame {
        assert_eq!(self.0.len(), 1, "expected exactly one frame");
        self.0.into_iter().next().unwrap()
    }

    /// Extract the frames.
    pub fn into_frames(self) -> Vec<TexFrame> {
        self.0
    }

    /// Iterate over the contained frames.
    pub fn iter(&self) -> std::slice::Iter<TexFrame> {
        self.0.iter()
    }

    /// Iterate over the contained frames.
    pub fn iter_mut(&mut self) -> std::slice::IterMut<TexFrame> {
        self.0.iter_mut()
    }
}

impl Debug for TexLayoutFragment {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.0.as_slice() {
            [frame] => frame.fmt(f),
            frames => frames.fmt(f),
        }
    }
}

impl IntoIterator for TexLayoutFragment {
    type Item = TexFrame;
    type IntoIter = std::vec::IntoIter<TexFrame>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a TexLayoutFragment {
    type Item = &'a TexFrame;
    type IntoIter = std::slice::Iter<'a, TexFrame>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a mut TexLayoutFragment {
    type Item = &'a mut TexFrame;
    type IntoIter = std::slice::IterMut<'a, TexFrame>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}
