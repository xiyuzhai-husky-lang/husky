use crate::*;
use husky_text_protocol::range::TextRange;
use husky_vfs::path::module_path::ModulePath;
use is::Is;

#[derive(Debug, PartialEq, Eq)]
pub struct InlayHint {}

// #[derive(Debug, Deserialize, Serialize)]
// pub struct InlayHint {
//     pub range: Range,
//     pub kind: InlayKind,
//     pub label: String,
// }
#[cfg(feature = "lsp_support")]
impl InlayHint {
    fn to_proto(&self) -> lsp_types::InlayHint {
        lsp_types::InlayHint {
            position: todo!(),
            label: todo!(),
            kind: todo!(),
            text_edits: todo!(),
            tooltip: todo!(),
            padding_left: None,
            padding_right: None,
            data: todo!(),
        }
    }
}

// todo: make this sealed
pub trait HasInlayHints: Is<ModulePath> + Copy {
    fn inlay_hints(
        self,
        db: &::salsa::Db,
        range: Option<TextRange>,
    ) -> InlayHintResult<Vec<InlayHint>>;

    fn lsp_inlay_hints(
        self,
        db: &::salsa::Db,
        range: Option<TextRange>,
    ) -> InlayHintResult<Option<Vec<lsp_types::InlayHint>>> {
        Ok(Some(
            self.inlay_hints(db, range)?
                .into_iter()
                .map(|inlay_hint| inlay_hint.to_proto())
                .collect::<Vec<_>>(),
        ))
    }
}

impl HasInlayHints for ModulePath {
    fn inlay_hints(
        self,
        db: &salsa::Db,
        range: Option<TextRange>,
    ) -> InlayHintResult<Vec<InlayHint>> {
        todo!()
    }
}
