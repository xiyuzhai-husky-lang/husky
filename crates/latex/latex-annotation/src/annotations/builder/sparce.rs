use crate::{
    annotation::{space::LxSpaceAnnotation, token::LxTokenAnnotation},
    annotations::LxAnnotations,
};
use husky_text::Text;
use latex_token::storage::LxTokenStorage;
use std::iter::Peekable;

pub struct LxAnnotationSparseBuilder<'a, I1, I2>
where
    I1: Iterator<Item = (&'a str, LxTokenAnnotation)>,
    I2: Iterator<Item = ((&'a str, &'a str), LxSpaceAnnotation)>,
{
    token_storage: &'a LxTokenStorage,
    text: Text<'a>,
    token_annotation_iter: Peekable<I1>,
    space_annotation_iter: Peekable<I2>,
    token_annotations: Vec<LxTokenAnnotation>,
    space_annotations: Vec<LxSpaceAnnotation>,
}

impl<'a, I1, I2> LxAnnotationSparseBuilder<'a, I1, I2>
where
    I1: Iterator<Item = (&'a str, LxTokenAnnotation)>,
    I2: Iterator<Item = ((&'a str, &'a str), LxSpaceAnnotation)>,
{
    pub fn new(
        token_storage: &'a LxTokenStorage,
        text: Text<'a>,
        token_annotation_iter: I1,
        space_annotation_iter: I2,
    ) -> Self {
        Self {
            token_storage,
            text,
            token_annotation_iter: token_annotation_iter.peekable(),
            space_annotation_iter: space_annotation_iter.peekable(),
            token_annotations: Vec::new(),
            space_annotations: Vec::new(),
        }
    }
}

impl<'a, I1, I2> LxAnnotationSparseBuilder<'a, I1, I2>
where
    I1: Iterator<Item = (&'a str, LxTokenAnnotation)>,
    I2: Iterator<Item = ((&'a str, &'a str), LxSpaceAnnotation)>,
{
    pub(crate) fn collect_all_annotations(&mut self) {
        self.collect_token_annotations();
        self.collect_space_annotations();
    }

    // for peek
    fn collect_token_annotations(&mut self) {
        for token_idx in self.token_storage.whole_token_idx_range() {
            let annotation = {
                let text_range = self.token_storage.token_text_range(token_idx);
                let text = self.text.text_within(text_range);
                match self.token_annotation_iter.peek() {
                    Some(&(s, _)) => {
                        if s == text {
                            self.token_annotation_iter.next().unwrap().1
                        } else {
                            LxTokenAnnotation::Void
                        }
                    }
                    None => LxTokenAnnotation::Void,
                }
            };
            self.token_annotations.push(annotation);
        }
    }

    fn collect_space_annotations(&mut self) {
        for token_idx in self.token_storage.whole_token_idx_range_without_the_first() {
            let prev_token_idx = token_idx - 1;
            let annotation = {
                let text_range = self.token_storage.token_text_range(token_idx);
                let prev_text_range = self.token_storage.token_text_range(prev_token_idx);
                let text = self.text.text_within(text_range);
                let prev_text = self.text.text_within(prev_text_range);
                match self.space_annotation_iter.peek() {
                    Some(&((s1, s2), _)) => {
                        if s1 == prev_text {
                            assert_eq!(s2, text);
                            self.space_annotation_iter.next().unwrap().1
                        } else {
                            LxSpaceAnnotation::Void
                        }
                    }
                    None => LxSpaceAnnotation::Void,
                }
            };
            self.space_annotations.push(annotation);
        }
    }

    pub(crate) fn finish(self) -> LxAnnotations {
        LxAnnotations::new(self.token_annotations, self.space_annotations)
    }
}
