use hir::{InFile, Semantics};
use husky_lang_db::{
    defs::Definition,
    vfs::{FileId, FilePosition, FileRange},
    helpers::visit_file_defs,
    HuskyLangDatabase,
};

use common::*;

use crate::{
    fn_references::find_all_methods, goto_implementation::goto_implementation,
    references::find_all_refs, NavigationTarget,
};

// Feature: Annotations
//
// Provides user with annotations above items for looking up references or impl blocks
// and running/debugging binaries.
//
// image::https://user-images.githubusercontent.com/48062697/113020672-b7c34f00-917a-11eb-8f6e-858735660a0e.png[]
#[derive(Debug)]
pub struct Annotation {
    pub range: TextRange,
    pub kind: AnnotationKind,
}

#[derive(Debug)]
pub enum AnnotationKind {
    HasImpls {
        position: FilePosition,
        data: Option<Vec<NavigationTarget>>,
    },
    HasReferences {
        position: FilePosition,
        data: Option<Vec<FileRange>>,
    },
}

pub struct AnnotationConfig {
    pub binary_target: bool,
    pub annotate_runnables: bool,
    pub annotate_impls: bool,
    pub annotate_references: bool,
    pub annotate_method_references: bool,
    pub annotate_enum_variant_references: bool,
}

pub(crate) fn annotations(
    db: &HuskyLangDatabase,
    config: &AnnotationConfig,
    file_id: FileId,
) -> Vec<Annotation> {
    todo!()
}

pub(crate) fn resolve_annotation(db: &HuskyLangDatabase, mut annotation: Annotation) -> Annotation {
    todo!()
}
