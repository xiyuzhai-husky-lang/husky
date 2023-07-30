// todo: use bitmap?
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TemplateParameterAttrs {
    phantom: bool,
}

pub enum TemplateParameterAttr {
    Phantom,
}
