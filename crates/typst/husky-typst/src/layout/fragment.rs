use std::fmt::{self, Debug, Formatter};

use crate::layout::TypstFrame;

/// A partial layout result.
#[derive(Clone)]
pub struct TypstLayoutFragment(Vec<TypstFrame>);

impl TypstLayoutFragment {
    /// Create a fragment from a single frame.
    pub fn frame(frame: TypstFrame) -> Self {
        Self(vec![frame])
    }

    /// Create a fragment from multiple frames.
    pub fn frames(frames: Vec<TypstFrame>) -> Self {
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
    pub fn into_frame(self) -> TypstFrame {
        assert_eq!(self.0.len(), 1, "expected exactly one frame");
        self.0.into_iter().next().unwrap()
    }

    /// Extract the frames.
    pub fn into_frames(self) -> Vec<TypstFrame> {
        self.0
    }

    /// Iterate over the contained frames.
    pub fn iter(&self) -> std::slice::Iter<TypstFrame> {
        self.0.iter()
    }

    /// Iterate over the contained frames.
    pub fn iter_mut(&mut self) -> std::slice::IterMut<TypstFrame> {
        self.0.iter_mut()
    }
}

impl Debug for TypstLayoutFragment {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.0.as_slice() {
            [frame] => frame.fmt(f),
            frames => frames.fmt(f),
        }
    }
}

impl IntoIterator for TypstLayoutFragment {
    type Item = TypstFrame;
    type IntoIter = std::vec::IntoIter<TypstFrame>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a TypstLayoutFragment {
    type Item = &'a TypstFrame;
    type IntoIter = std::slice::Iter<'a, TypstFrame>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a mut TypstLayoutFragment {
    type Item = &'a mut TypstFrame;
    type IntoIter = std::slice::IterMut<'a, TypstFrame>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}
