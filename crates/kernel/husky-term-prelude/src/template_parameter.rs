// todo: use bitmap?
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct TemplateParameterAttrs {
    phantom: bool,
}

impl TemplateParameterAttrs {
    pub fn from_syn_attrs<I: Into<TemplateParameterAttr>>(
        syn_attrs: impl IntoIterator<Item = I>,
    ) -> Self {
        let mut this: Self = Default::default();
        for syn_attr in syn_attrs {
            todo!()
        }
        this
    }
}

pub enum TemplateParameterAttr {
    Phantom,
}
