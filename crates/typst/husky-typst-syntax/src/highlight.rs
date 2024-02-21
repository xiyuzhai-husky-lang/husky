use crate::{ast, LinkedNode, TypstSyntaxKind, TypstSyntaxNode};

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
            Self::Comment => "comment.typst",
            Self::Punctuation => "punctuation.typst",
            Self::Escape => "constant.character.escape.typst",
            Self::Strong => "markup.bold.typst",
            Self::Emph => "markup.italic.typst",
            Self::Link => "markup.underline.link.typst",
            Self::Raw => "markup.raw.typst",
            Self::MathDelimiter => "punctuation.definition.math.typst",
            Self::MathOperator => "keyword.operator.math.typst",
            Self::Heading => "markup.heading.typst",
            Self::ListMarker => "punctuation.definition.list.typst",
            Self::ListTerm => "markup.list.term.typst",
            Self::Label => "entity.name.label.typst",
            Self::Ref => "markup.other.reference.typst",
            Self::Keyword => "keyword.typst",
            Self::Operator => "keyword.operator.typst",
            Self::Number => "constant.numeric.typst",
            Self::String => "string.quoted.double.typst",
            Self::Function => "entity.name.function.typst",
            Self::Interpolated => "meta.interpolation.typst",
            Self::Error => "invalid.typst",
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
        TypstSyntaxKind::TypstMarkup
            if node.parent_kind() == Some(TypstSyntaxKind::TermItem)
                && node.next_sibling_kind() == Some(TypstSyntaxKind::Colon) =>
        {
            Some(Tag::ListTerm)
        }
        TypstSyntaxKind::TypstMarkup => None,
        TypstSyntaxKind::Text => None,
        TypstSyntaxKind::Space => None,
        TypstSyntaxKind::Linebreak => Some(Tag::Escape),
        TypstSyntaxKind::Parbreak => None,
        TypstSyntaxKind::Escape => Some(Tag::Escape),
        TypstSyntaxKind::Shorthand => Some(Tag::Escape),
        TypstSyntaxKind::SmartQuote => None,
        TypstSyntaxKind::Strong => Some(Tag::Strong),
        TypstSyntaxKind::Emph => Some(Tag::Emph),
        TypstSyntaxKind::Raw => Some(Tag::Raw),
        TypstSyntaxKind::Link => Some(Tag::Link),
        TypstSyntaxKind::Label => Some(Tag::Label),
        TypstSyntaxKind::Ref => Some(Tag::Ref),
        TypstSyntaxKind::RefMarker => None,
        TypstSyntaxKind::Heading => Some(Tag::Heading),
        TypstSyntaxKind::HeadingMarker => None,
        TypstSyntaxKind::ListItem => None,
        TypstSyntaxKind::ListMarker => Some(Tag::ListMarker),
        TypstSyntaxKind::EnumItem => None,
        TypstSyntaxKind::EnumMarker => Some(Tag::ListMarker),
        TypstSyntaxKind::TermItem => None,
        TypstSyntaxKind::TermMarker => Some(Tag::ListMarker),
        TypstSyntaxKind::Equation => None,

        TypstSyntaxKind::Math => None,
        TypstSyntaxKind::MathIdent => highlight_ident(node),
        TypstSyntaxKind::MathAlignPoint => Some(Tag::MathOperator),
        TypstSyntaxKind::MathDelimited => None,
        TypstSyntaxKind::MathAttach => None,
        TypstSyntaxKind::MathFrac => None,
        TypstSyntaxKind::MathRoot => None,
        TypstSyntaxKind::MathPrimes => None,

        TypstSyntaxKind::Hash => highlight_hash(node),
        TypstSyntaxKind::LeftBrace => Some(Tag::Punctuation),
        TypstSyntaxKind::RightBrace => Some(Tag::Punctuation),
        TypstSyntaxKind::LeftDelimiter => Some(Tag::Punctuation),
        TypstSyntaxKind::RightDelimiter => Some(Tag::Punctuation),
        TypstSyntaxKind::LeftParen => Some(Tag::Punctuation),
        TypstSyntaxKind::RightParen => Some(Tag::Punctuation),
        TypstSyntaxKind::Comma => Some(Tag::Punctuation),
        TypstSyntaxKind::Semicolon => Some(Tag::Punctuation),
        TypstSyntaxKind::Colon => Some(Tag::Punctuation),
        TypstSyntaxKind::Star => match node.parent_kind() {
            Some(TypstSyntaxKind::Strong) => None,
            _ => Some(Tag::Operator),
        },
        TypstSyntaxKind::Underscore => match node.parent_kind() {
            Some(TypstSyntaxKind::MathAttach) => Some(Tag::MathOperator),
            _ => None,
        },
        TypstSyntaxKind::Dollar => Some(Tag::MathDelimiter),
        TypstSyntaxKind::Plus => Some(Tag::Operator),
        TypstSyntaxKind::Minus => Some(Tag::Operator),
        TypstSyntaxKind::Slash => Some(match node.parent_kind() {
            Some(TypstSyntaxKind::MathFrac) => Tag::MathOperator,
            _ => Tag::Operator,
        }),
        TypstSyntaxKind::Hat => Some(Tag::MathOperator),
        TypstSyntaxKind::Prime => Some(Tag::MathOperator),
        TypstSyntaxKind::Dot => Some(Tag::Punctuation),
        TypstSyntaxKind::Eq => match node.parent_kind() {
            Some(TypstSyntaxKind::Heading) => None,
            _ => Some(Tag::Operator),
        },
        TypstSyntaxKind::EqEq => Some(Tag::Operator),
        TypstSyntaxKind::ExclEq => Some(Tag::Operator),
        TypstSyntaxKind::Lt => Some(Tag::Operator),
        TypstSyntaxKind::LtEq => Some(Tag::Operator),
        TypstSyntaxKind::Gt => Some(Tag::Operator),
        TypstSyntaxKind::GtEq => Some(Tag::Operator),
        TypstSyntaxKind::PlusEq => Some(Tag::Operator),
        TypstSyntaxKind::HyphEq => Some(Tag::Operator),
        TypstSyntaxKind::StarEq => Some(Tag::Operator),
        TypstSyntaxKind::SlashEq => Some(Tag::Operator),
        TypstSyntaxKind::Dots => Some(Tag::Operator),
        TypstSyntaxKind::Arrow => Some(Tag::Operator),
        TypstSyntaxKind::Root => Some(Tag::MathOperator),

        TypstSyntaxKind::Not => Some(Tag::Keyword),
        TypstSyntaxKind::And => Some(Tag::Keyword),
        TypstSyntaxKind::Or => Some(Tag::Keyword),
        TypstSyntaxKind::None => Some(Tag::Keyword),
        TypstSyntaxKind::Auto => Some(Tag::Keyword),
        TypstSyntaxKind::Let => Some(Tag::Keyword),
        TypstSyntaxKind::Set => Some(Tag::Keyword),
        TypstSyntaxKind::Show => Some(Tag::Keyword),
        TypstSyntaxKind::If => Some(Tag::Keyword),
        TypstSyntaxKind::Else => Some(Tag::Keyword),
        TypstSyntaxKind::For => Some(Tag::Keyword),
        TypstSyntaxKind::In => Some(Tag::Keyword),
        TypstSyntaxKind::While => Some(Tag::Keyword),
        TypstSyntaxKind::Break => Some(Tag::Keyword),
        TypstSyntaxKind::Continue => Some(Tag::Keyword),
        TypstSyntaxKind::Return => Some(Tag::Keyword),
        TypstSyntaxKind::Import => Some(Tag::Keyword),
        TypstSyntaxKind::Include => Some(Tag::Keyword),
        TypstSyntaxKind::As => Some(Tag::Keyword),

        TypstSyntaxKind::Code => None,
        TypstSyntaxKind::Ident => highlight_ident(node),
        TypstSyntaxKind::Bool => Some(Tag::Keyword),
        TypstSyntaxKind::Int => Some(Tag::Number),
        TypstSyntaxKind::Float => Some(Tag::Number),
        TypstSyntaxKind::Numeric => Some(Tag::Number),
        TypstSyntaxKind::Str => Some(Tag::String),
        TypstSyntaxKind::CodeBlock => None,
        TypstSyntaxKind::ContentBlock => None,
        TypstSyntaxKind::Parenthesized => None,
        TypstSyntaxKind::Array => None,
        TypstSyntaxKind::Dict => None,
        TypstSyntaxKind::Named => None,
        TypstSyntaxKind::Keyed => None,
        TypstSyntaxKind::Unary => None,
        TypstSyntaxKind::Binary => None,
        TypstSyntaxKind::FieldAccess => None,
        TypstSyntaxKind::FuncCall => None,
        TypstSyntaxKind::Args => None,
        TypstSyntaxKind::Spread => None,
        TypstSyntaxKind::Closure => None,
        TypstSyntaxKind::Params => None,
        TypstSyntaxKind::LetBinding => None,
        TypstSyntaxKind::SetRule => None,
        TypstSyntaxKind::ShowRule => None,
        TypstSyntaxKind::Conditional => None,
        TypstSyntaxKind::WhileLoop => None,
        TypstSyntaxKind::ForLoop => None,
        TypstSyntaxKind::ModuleImport => None,
        TypstSyntaxKind::ImportItems => None,
        TypstSyntaxKind::RenamedImportItem => None,
        TypstSyntaxKind::ModuleInclude => None,
        TypstSyntaxKind::LoopBreak => None,
        TypstSyntaxKind::LoopContinue => None,
        TypstSyntaxKind::FuncReturn => None,
        TypstSyntaxKind::Destructuring => None,
        TypstSyntaxKind::DestructAssignment => None,

        TypstSyntaxKind::LineComment => Some(Tag::Comment),
        TypstSyntaxKind::BlockComment => Some(Tag::Comment),
        TypstSyntaxKind::Error => Some(Tag::Error),
        TypstSyntaxKind::Eof => None,
    }
}

/// Highlight an identifier based on context.
fn highlight_ident(node: &LinkedNode) -> Option<Tag> {
    // Are we directly before an argument list?
    let next_leaf = node.next_leaf();
    if let Some(next) = &next_leaf {
        if node.range().end == next.offset()
            && ((next.kind() == TypstSyntaxKind::LeftParen
                && matches!(
                    next.parent_kind(),
                    Some(TypstSyntaxKind::Args | TypstSyntaxKind::Params)
                ))
                || (next.kind() == TypstSyntaxKind::LeftDelimiter
                    && next.parent_kind() == Some(TypstSyntaxKind::ContentBlock)))
        {
            return Some(Tag::Function);
        }
    }

    // Are we in math?
    if node.kind() == TypstSyntaxKind::MathIdent {
        return Some(Tag::Interpolated);
    }

    // Find the first non-field access ancestor.
    let mut ancestor = node;
    while ancestor.parent_kind() == Some(TypstSyntaxKind::FieldAccess) {
        ancestor = ancestor.parent()?;
    }

    // Are we directly before or behind a show rule colon?
    if ancestor.parent_kind() == Some(TypstSyntaxKind::ShowRule)
        && (next_leaf.map(|leaf| leaf.kind()) == Some(TypstSyntaxKind::Colon)
            || node.prev_leaf().map(|leaf| leaf.kind()) == Some(TypstSyntaxKind::Colon))
    {
        return Some(Tag::Function);
    }

    // Are we (or an ancestor field access) directly after a hash.
    if ancestor.prev_leaf().map(|leaf| leaf.kind()) == Some(TypstSyntaxKind::Hash) {
        return Some(Tag::Interpolated);
    }

    // Are we behind a dot, that is behind another identifier?
    let prev = node.prev_leaf()?;
    if prev.kind() == TypstSyntaxKind::Dot {
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
    matches!(
        node.kind(),
        TypstSyntaxKind::Ident | TypstSyntaxKind::MathIdent
    )
}

/// Highlight a node to an HTML `code` element.
///
/// This uses these [CSS classes for categories](Tag::css_class).
pub fn highlight_html(root: &TypstSyntaxNode) -> String {
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
