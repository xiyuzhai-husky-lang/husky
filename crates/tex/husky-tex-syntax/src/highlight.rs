use crate::{ast, LinkedNode, TexSyntaxKind, TexSyntaxNode};

/// A syntax highlighting tag.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Tag {
    /// A line or block comment.
    Comment,
    /// Punctuation in code.
    Punctuation,
    /// An escape sequence or shorthand.
    Escape,
    /// Strong markup.
    Strong,
    /// Emphasized markup.
    Emph,
    /// A hyperlink.
    Link,
    /// Raw text.
    Raw,
    /// A label.
    Label,
    /// A reference to a label.
    Ref,
    /// A section heading.
    Heading,
    /// A marker of a list, enumeration, or term list.
    ListMarker,
    /// A term in a term list.
    ListTerm,
    /// The delimiters of an equation.
    MathDelimiter,
    /// An operator with special meaning in an equation.
    MathOperator,
    /// A keyword.
    Keyword,
    /// An operator in code.
    Operator,
    /// A numeric literal.
    Number,
    /// A string literal.
    String,
    /// A function or method name.
    Function,
    /// An interpolated variable in markup or math.
    Interpolated,
    /// A syntax error.
    Error,
}

impl Tag {
    /// The list of all tags, in the same order as thy are defined.
    ///
    /// Can be used as the counter-part to `tag as usize`.
    pub const LIST: &'static [Tag] = &[
        Self::Comment,
        Self::Punctuation,
        Self::Escape,
        Self::Strong,
        Self::Emph,
        Self::Link,
        Self::Raw,
        Self::Label,
        Self::Ref,
        Self::Heading,
        Self::ListMarker,
        Self::ListTerm,
        Self::MathDelimiter,
        Self::MathOperator,
        Self::Keyword,
        Self::Operator,
        Self::Number,
        Self::String,
        Self::Function,
        Self::Interpolated,
        Self::Error,
    ];

    /// Return the recommended TextMate grammar scope for the given highlighting
    /// tag.
    pub fn tm_scope(&self) -> &'static str {
        match self {
            Self::Comment => "comment.tex",
            Self::Punctuation => "punctuation.tex",
            Self::Escape => "constant.character.escape.tex",
            Self::Strong => "markup.bold.tex",
            Self::Emph => "markup.italic.tex",
            Self::Link => "markup.underline.link.tex",
            Self::Raw => "markup.raw.tex",
            Self::MathDelimiter => "punctuation.definition.math.tex",
            Self::MathOperator => "keyword.operator.math.tex",
            Self::Heading => "markup.heading.tex",
            Self::ListMarker => "punctuation.definition.list.tex",
            Self::ListTerm => "markup.list.term.tex",
            Self::Label => "entity.name.label.tex",
            Self::Ref => "markup.other.reference.tex",
            Self::Keyword => "keyword.tex",
            Self::Operator => "keyword.operator.tex",
            Self::Number => "constant.numeric.tex",
            Self::String => "string.quoted.double.tex",
            Self::Function => "entity.name.function.tex",
            Self::Interpolated => "meta.interpolation.tex",
            Self::Error => "invalid.tex",
        }
    }

    /// The recommended CSS class for the highlighting tag.
    pub fn css_class(self) -> &'static str {
        match self {
            Self::Comment => "typ-comment",
            Self::Punctuation => "typ-punct",
            Self::Escape => "typ-escape",
            Self::Strong => "typ-strong",
            Self::Emph => "typ-emph",
            Self::Link => "typ-link",
            Self::Raw => "typ-raw",
            Self::Label => "typ-label",
            Self::Ref => "typ-ref",
            Self::Heading => "typ-heading",
            Self::ListMarker => "typ-marker",
            Self::ListTerm => "typ-term",
            Self::MathDelimiter => "typ-math-delim",
            Self::MathOperator => "typ-math-op",
            Self::Keyword => "typ-key",
            Self::Operator => "typ-op",
            Self::Number => "typ-num",
            Self::String => "typ-str",
            Self::Function => "typ-func",
            Self::Interpolated => "typ-pol",
            Self::Error => "typ-error",
        }
    }
}

/// Determine the highlight tag of a linked syntax node.
///
/// Returns `None` if the node should not be highlighted.
pub fn highlight(node: &LinkedNode) -> Option<Tag> {
    match node.kind() {
        TexSyntaxKind::TexMarkup
            if node.parent_kind() == Some(TexSyntaxKind::TermItem)
                && node.next_sibling_kind() == Some(TexSyntaxKind::Colon) =>
        {
            Some(Tag::ListTerm)
        }
        TexSyntaxKind::TexMarkup => None,
        TexSyntaxKind::Text => None,
        TexSyntaxKind::Space => None,
        TexSyntaxKind::Linebreak => Some(Tag::Escape),
        TexSyntaxKind::Parbreak => None,
        TexSyntaxKind::Escape => Some(Tag::Escape),
        TexSyntaxKind::Shorthand => Some(Tag::Escape),
        TexSyntaxKind::SmartQuote => None,
        TexSyntaxKind::Strong => Some(Tag::Strong),
        TexSyntaxKind::Emph => Some(Tag::Emph),
        TexSyntaxKind::Raw => Some(Tag::Raw),
        TexSyntaxKind::Link => Some(Tag::Link),
        TexSyntaxKind::Label => Some(Tag::Label),
        TexSyntaxKind::Ref => Some(Tag::Ref),
        TexSyntaxKind::RefMarker => None,
        TexSyntaxKind::Heading => Some(Tag::Heading),
        TexSyntaxKind::HeadingMarker => None,
        TexSyntaxKind::ListItem => None,
        TexSyntaxKind::ListMarker => Some(Tag::ListMarker),
        TexSyntaxKind::EnumItem => None,
        TexSyntaxKind::EnumMarker => Some(Tag::ListMarker),
        TexSyntaxKind::TermItem => None,
        TexSyntaxKind::TermMarker => Some(Tag::ListMarker),
        TexSyntaxKind::Equation => None,

        TexSyntaxKind::Math => None,
        TexSyntaxKind::MathIdent => highlight_ident(node),
        TexSyntaxKind::MathAlignPoint => Some(Tag::MathOperator),
        TexSyntaxKind::MathDelimited => None,
        TexSyntaxKind::MathAttach => None,
        TexSyntaxKind::MathFrac => None,
        TexSyntaxKind::MathRoot => None,
        TexSyntaxKind::MathPrimes => None,

        TexSyntaxKind::Hash => highlight_hash(node),
        TexSyntaxKind::LeftBrace => Some(Tag::Punctuation),
        TexSyntaxKind::RightBrace => Some(Tag::Punctuation),
        TexSyntaxKind::LeftDelimiter => Some(Tag::Punctuation),
        TexSyntaxKind::RightDelimiter => Some(Tag::Punctuation),
        TexSyntaxKind::LeftParen => Some(Tag::Punctuation),
        TexSyntaxKind::RightParen => Some(Tag::Punctuation),
        TexSyntaxKind::Comma => Some(Tag::Punctuation),
        TexSyntaxKind::Semicolon => Some(Tag::Punctuation),
        TexSyntaxKind::Colon => Some(Tag::Punctuation),
        TexSyntaxKind::Star => match node.parent_kind() {
            Some(TexSyntaxKind::Strong) => None,
            _ => Some(Tag::Operator),
        },
        TexSyntaxKind::Underscore => match node.parent_kind() {
            Some(TexSyntaxKind::MathAttach) => Some(Tag::MathOperator),
            _ => None,
        },
        TexSyntaxKind::Dollar => Some(Tag::MathDelimiter),
        TexSyntaxKind::Plus => Some(Tag::Operator),
        TexSyntaxKind::Minus => Some(Tag::Operator),
        TexSyntaxKind::Slash => Some(match node.parent_kind() {
            Some(TexSyntaxKind::MathFrac) => Tag::MathOperator,
            _ => Tag::Operator,
        }),
        TexSyntaxKind::Hat => Some(Tag::MathOperator),
        TexSyntaxKind::Prime => Some(Tag::MathOperator),
        TexSyntaxKind::Dot => Some(Tag::Punctuation),
        TexSyntaxKind::Eq => match node.parent_kind() {
            Some(TexSyntaxKind::Heading) => None,
            _ => Some(Tag::Operator),
        },
        TexSyntaxKind::EqEq => Some(Tag::Operator),
        TexSyntaxKind::ExclEq => Some(Tag::Operator),
        TexSyntaxKind::Lt => Some(Tag::Operator),
        TexSyntaxKind::LtEq => Some(Tag::Operator),
        TexSyntaxKind::Gt => Some(Tag::Operator),
        TexSyntaxKind::GtEq => Some(Tag::Operator),
        TexSyntaxKind::PlusEq => Some(Tag::Operator),
        TexSyntaxKind::HyphEq => Some(Tag::Operator),
        TexSyntaxKind::StarEq => Some(Tag::Operator),
        TexSyntaxKind::SlashEq => Some(Tag::Operator),
        TexSyntaxKind::Dots => Some(Tag::Operator),
        TexSyntaxKind::Arrow => Some(Tag::Operator),
        TexSyntaxKind::Root => Some(Tag::MathOperator),

        TexSyntaxKind::Not => Some(Tag::Keyword),
        TexSyntaxKind::And => Some(Tag::Keyword),
        TexSyntaxKind::Or => Some(Tag::Keyword),
        TexSyntaxKind::None => Some(Tag::Keyword),
        TexSyntaxKind::Auto => Some(Tag::Keyword),
        TexSyntaxKind::Let => Some(Tag::Keyword),
        TexSyntaxKind::Set => Some(Tag::Keyword),
        TexSyntaxKind::Show => Some(Tag::Keyword),
        TexSyntaxKind::If => Some(Tag::Keyword),
        TexSyntaxKind::Else => Some(Tag::Keyword),
        TexSyntaxKind::For => Some(Tag::Keyword),
        TexSyntaxKind::In => Some(Tag::Keyword),
        TexSyntaxKind::While => Some(Tag::Keyword),
        TexSyntaxKind::Break => Some(Tag::Keyword),
        TexSyntaxKind::Continue => Some(Tag::Keyword),
        TexSyntaxKind::Return => Some(Tag::Keyword),
        TexSyntaxKind::Import => Some(Tag::Keyword),
        TexSyntaxKind::Include => Some(Tag::Keyword),
        TexSyntaxKind::As => Some(Tag::Keyword),

        TexSyntaxKind::Code => None,
        TexSyntaxKind::Ident => highlight_ident(node),
        TexSyntaxKind::Bool => Some(Tag::Keyword),
        TexSyntaxKind::Int => Some(Tag::Number),
        TexSyntaxKind::Float => Some(Tag::Number),
        TexSyntaxKind::Numeric => Some(Tag::Number),
        TexSyntaxKind::Str => Some(Tag::String),
        TexSyntaxKind::CodeBlock => None,
        TexSyntaxKind::ContentBlock => None,
        TexSyntaxKind::Parenthesized => None,
        TexSyntaxKind::Array => None,
        TexSyntaxKind::Dict => None,
        TexSyntaxKind::Named => None,
        TexSyntaxKind::Keyed => None,
        TexSyntaxKind::Unary => None,
        TexSyntaxKind::Binary => None,
        TexSyntaxKind::FieldAccess => None,
        TexSyntaxKind::FuncCall => None,
        TexSyntaxKind::Args => None,
        TexSyntaxKind::Spread => None,
        TexSyntaxKind::Closure => None,
        TexSyntaxKind::Params => None,
        TexSyntaxKind::LetBinding => None,
        TexSyntaxKind::SetRule => None,
        TexSyntaxKind::ShowRule => None,
        TexSyntaxKind::Conditional => None,
        TexSyntaxKind::WhileLoop => None,
        TexSyntaxKind::ForLoop => None,
        TexSyntaxKind::ModuleImport => None,
        TexSyntaxKind::ImportItems => None,
        TexSyntaxKind::RenamedImportItem => None,
        TexSyntaxKind::ModuleInclude => None,
        TexSyntaxKind::LoopBreak => None,
        TexSyntaxKind::LoopContinue => None,
        TexSyntaxKind::FuncReturn => None,
        TexSyntaxKind::Destructuring => None,
        TexSyntaxKind::DestructAssignment => None,

        TexSyntaxKind::LineComment => Some(Tag::Comment),
        TexSyntaxKind::BlockComment => Some(Tag::Comment),
        TexSyntaxKind::Error => Some(Tag::Error),
        TexSyntaxKind::Eof => None,
    }
}

/// Highlight an identifier based on context.
fn highlight_ident(node: &LinkedNode) -> Option<Tag> {
    // Are we directly before an argument list?
    let next_leaf = node.next_leaf();
    if let Some(next) = &next_leaf {
        if node.range().end == next.offset()
            && ((next.kind() == TexSyntaxKind::LeftParen
                && matches!(
                    next.parent_kind(),
                    Some(TexSyntaxKind::Args | TexSyntaxKind::Params)
                ))
                || (next.kind() == TexSyntaxKind::LeftDelimiter
                    && next.parent_kind() == Some(TexSyntaxKind::ContentBlock)))
        {
            return Some(Tag::Function);
        }
    }

    // Are we in math?
    if node.kind() == TexSyntaxKind::MathIdent {
        return Some(Tag::Interpolated);
    }

    // Find the first non-field access ancestor.
    let mut ancestor = node;
    while ancestor.parent_kind() == Some(TexSyntaxKind::FieldAccess) {
        ancestor = ancestor.parent()?;
    }

    // Are we directly before or behind a show rule colon?
    if ancestor.parent_kind() == Some(TexSyntaxKind::ShowRule)
        && (next_leaf.map(|leaf| leaf.kind()) == Some(TexSyntaxKind::Colon)
            || node.prev_leaf().map(|leaf| leaf.kind()) == Some(TexSyntaxKind::Colon))
    {
        return Some(Tag::Function);
    }

    // Are we (or an ancestor field access) directly after a hash.
    if ancestor.prev_leaf().map(|leaf| leaf.kind()) == Some(TexSyntaxKind::Hash) {
        return Some(Tag::Interpolated);
    }

    // Are we behind a dot, that is behind another identifier?
    let prev = node.prev_leaf()?;
    if prev.kind() == TexSyntaxKind::Dot {
        let prev_prev = prev.prev_leaf()?;
        if is_ident(&prev_prev) {
            return highlight_ident(&prev_prev);
        }
    }

    None
}

/// Highlight a hash based on context.
fn highlight_hash(node: &LinkedNode) -> Option<Tag> {
    let next = node.next_sibling()?;
    let expr = next.cast::<ast::Expr>()?;
    if !expr.hash() {
        return None;
    }
    highlight(&next.leftmost_leaf()?)
}

/// Whether the node is one of the two identifier nodes.
fn is_ident(node: &LinkedNode) -> bool {
    matches!(node.kind(), TexSyntaxKind::Ident | TexSyntaxKind::MathIdent)
}

/// Highlight a node to an HTML `code` element.
///
/// This uses these [CSS classes for categories](Tag::css_class).
pub fn highlight_html(root: &TexSyntaxNode) -> String {
    let mut buf = String::from("<code>");
    let node = LinkedNode::new(root);
    highlight_html_impl(&mut buf, &node);
    buf.push_str("</code>");
    buf
}

/// Highlight one source node, emitting HTML.
fn highlight_html_impl(html: &mut String, node: &LinkedNode) {
    let mut span = false;
    if let Some(tag) = highlight(node) {
        if tag != Tag::Error {
            span = true;
            html.push_str("<span class=\"");
            html.push_str(tag.css_class());
            html.push_str("\">");
        }
    }

    let text = node.text();
    if !text.is_empty() {
        for c in text.chars() {
            match c {
                '<' => html.push_str("&lt;"),
                '>' => html.push_str("&gt;"),
                '&' => html.push_str("&amp;"),
                '\'' => html.push_str("&#39;"),
                '"' => html.push_str("&quot;"),
                _ => html.push(c),
            }
        }
    } else {
        for child in node.children() {
            highlight_html_impl(html, &child);
        }
    }

    if span {
        html.push_str("</span>");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Range;

    #[test]
    fn test_highlighting() {
        use Tag::*;

        #[track_caller]
        fn test(text: &str, goal: &[(Range<usize>, Tag)]) {
            let mut vec = vec![];
            let root = crate::parse(text);
            highlight_tree(&mut vec, &LinkedNode::new(&root));
            assert_eq!(vec, goal);
        }

        fn highlight_tree(tags: &mut Vec<(Range<usize>, Tag)>, node: &LinkedNode) {
            if let Some(tag) = highlight(node) {
                tags.push((node.range(), tag));
            }

            for child in node.children() {
                highlight_tree(tags, &child);
            }
        }

        test("= *AB*", &[(0..6, Heading), (2..6, Strong)]);

        test(
            "#f(x + 1)",
            &[
                (0..1, Function),
                (1..2, Function),
                (2..3, Punctuation),
                (5..6, Operator),
                (7..8, Number),
                (8..9, Punctuation),
            ],
        );

        test(
            "#let f(x) = x",
            &[
                (0..1, Keyword),
                (1..4, Keyword),
                (5..6, Function),
                (6..7, Punctuation),
                (8..9, Punctuation),
                (10..11, Operator),
            ],
        );
    }
}
