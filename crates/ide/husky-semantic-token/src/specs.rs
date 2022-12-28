use crate::*;

macro_rules! define_semantic_token_types {
    ($(($ident:ident, $string:literal)),*$(,)?) => {
        $(pub  const $ident: ext::SemanticTokenType = ext::SemanticTokenType::new($string);)*

        pub const SUPPORTED_TYPES: &[ext::SemanticTokenType] = &[
            ext::SemanticTokenType::COMMENT,
            ext::SemanticTokenType::DECORATOR,
            ext::SemanticTokenType::KEYWORD,
            ext::SemanticTokenType::STRING,
            ext::SemanticTokenType::NUMBER,
            ext::SemanticTokenType::REGEXP,
            ext::SemanticTokenType::OPERATOR,
            ext::SemanticTokenType::NAMESPACE,
            ext::SemanticTokenType::TYPE,
            ext::SemanticTokenType::STRUCT,
            ext::SemanticTokenType::CLASS,
            ext::SemanticTokenType::INTERFACE,
            ext::SemanticTokenType::ENUM,
            ext::SemanticTokenType::ENUM_MEMBER,
            ext::SemanticTokenType::TYPE_PARAMETER,
            ext::SemanticTokenType::FUNCTION,
            ext::SemanticTokenType::METHOD,
            ext::SemanticTokenType::PROPERTY,
            ext::SemanticTokenType::MACRO,
            ext::SemanticTokenType::VARIABLE,
            ext::SemanticTokenType::PARAMETER,
            $($ident),*
        ];
    };
    }

define_semantic_token_types![
    (ANGLE, "angle"),
    (ARITHMETIC, "arithmetic"),
    (ATTRIBUTE, "attribute"),
    (BITWISE, "bitwise"),
    (BOOLEAN, "boolean"),
    (BRACE, "brace"),
    (BRACKET, "bracket"),
    (BUILTIN_ATTRIBUTE, "builtinAttribute"),
    (BUILTIN_TYPE, "builtinType"),
    (CHAR, "character"),
    (COLON, "colon"),
    (COMMA, "comma"),
    (COMPARISON, "comparison"),
    (CONST_PARAMETER, "constParameter"),
    (DOT, "dot"),
    (ESCAPE_SEQUENCE, "escapeSequence"),
    (FORMAT_SPECIFIER, "formatSpecifier"),
    (GENERIC, "generic"),
    (LABEL, "label"),
    (LIFETIME, "lifetime"),
    (LOGICAL, "logical"),
    (OPERATOR, "operator"),
    (PARENTHESIS, "parenthesis"),
    (PUNCTUATION, "punctuation"),
    (SELF_KEYWORD, "selfKeyword"),
    (SEMICOLON, "semicolon"),
    (TYPE_ALIAS, "typeAlias"),
    (UNION, "union"),
    (UNRESOLVED_REFERENCE, "unresolvedReference"),
];

macro_rules! define_semantic_token_modifiers {
    ($(($ident:ident, $string:literal)),*$(,)?) => {
        $(pub const $ident: ext::SemanticTokenModifier = ext::SemanticTokenModifier::new($string);)*

        pub const SUPPORTED_MODIFIERS: &[ext::SemanticTokenModifier] = &[
            ext::SemanticTokenModifier::DOCUMENTATION,
            ext::SemanticTokenModifier::DECLARATION,
            ext::SemanticTokenModifier::DEFINITION,
            ext::SemanticTokenModifier::STATIC,
            ext::SemanticTokenModifier::ABSTRACT,
            ext::SemanticTokenModifier::DEPRECATED,
            ext::SemanticTokenModifier::READONLY,
            ext::SemanticTokenModifier::DEFAULT_LIBRARY,
            $($ident),*
        ];
    };
    }

define_semantic_token_modifiers![
    (ASYNC, "async"),
    (ATTRIBUTE_MODIFIER, "attribute"),
    (CALLABLE, "callable"),
    (CONSTANT, "constant"),
    (CONSUMING, "consuming"),
    (CONTROL_FLOW, "controlFlow"),
    (CRATE_ROOT, "crateRoot"),
    (INJECTED, "injected"),
    (INTRA_DOC_LINK, "intraDocLink"),
    (LIBRARY, "library"),
    (MUTABLE, "mutable"),
    (PUBLIC, "public"),
    (REFERENCE, "reference"),
    (TRAIT_MODIFIER, "trait"),
    (UNSAFE, "unsafe"),
];
