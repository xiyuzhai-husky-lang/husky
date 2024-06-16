use crate::*;
use husky_text_protocol::range::TextRange;
use husky_vfs::path::module_path::ModulePath;
use is::Is;

#[derive(Debug, PartialEq, Eq)]
pub struct InlayHint {
    label: InlayHintLabel,
    kind: InlayHintKind,
}

#[derive(Debug, PartialEq, Eq)]
pub enum InlayHintLabel {
    String(String),
    LabelParts(Vec<InlayHintLabelPart>),
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct InlayHintLabelPart {
    /// The value of this label part.
    pub value: String,

    /// The tooltip text when you hover over this label part. Depending on
    /// the client capability `inlayHint.resolveSupport` clients might resolve
    /// this property late using the resolve request.
    pub tooltip: Option<InlayHintLabelPartTooltip>,

    /// An optional source code location that represents this
    /// label part.
    ///
    /// The editor will use this location for the hover and for code navigation
    /// features: This part will become a clickable link that resolves to the
    /// definition of the symbol at the given location (not necessarily the
    /// location itself), it shows the hover that shows at the given location,
    /// and it shows a context menu with further code navigation commands.
    ///
    /// Depending on the client capability `inlayHint.resolveSupport` clients
    /// might resolve this property late using the resolve request.
    pub location: Option<InlayHintLocation>,

    /// An optional command for this label part.
    ///
    /// Depending on the client capability `inlayHint.resolveSupport` clients
    /// might resolve this property late using the resolve request.
    pub command: Option<Command>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct InlayHintLabelPartTooltip;

#[derive(Debug, PartialEq, Eq)]
pub struct InlayHintLocation {
    module_path: ModulePath,
    range: TextRange,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Command {
    /// Title of the command, like `save`.
    pub title: String,
    /// The identifier of the actual command handler.
    pub command: String,
    /// Arguments that the command handler should be
    /// invoked with.
    pub arguments: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InlayHintKind {
    TypeHint,
    ParameterHint,
    ChainingHint,
}

#[cfg(feature = "lsp_support")]
impl InlayHint {
    fn to_proto(&self) -> lsp_types::InlayHint {
        lsp_types::InlayHint {
            position: todo!(),
            label: todo!(),
            kind: self.kind.into(),
            text_edits: None,
            tooltip: None,
            padding_left: None,
            padding_right: None,
            data: None,
        }
    }
}

#[cfg(feature = "lsp_support")]
impl Into<Option<lsp_types::InlayHintKind>> for InlayHintKind {
    fn into(self) -> Option<lsp_types::InlayHintKind> {
        match self {
            InlayHintKind::TypeHint => Some(lsp_types::InlayHintKind::TYPE),
            InlayHintKind::ParameterHint => Some(lsp_types::InlayHintKind::PARAMETER),
            InlayHintKind::ChainingHint => None,
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
