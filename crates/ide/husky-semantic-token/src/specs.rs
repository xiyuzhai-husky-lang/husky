use crate::*;

macro_rules! define_semantic_token_types {
    ($(($ident:ident, $string:literal)),*$(,)?) => {
        $(pub  const $ident: SemanticTokenType = SemanticTokenType::new($string);)*

        pub const SUPPORTED_TYPES: &[SemanticTokenType] = &[
            SemanticTokenType::COMMENT,
            SemanticTokenType::KEYWORD,
            SemanticTokenType::STRING,
            SemanticTokenType::NUMBER,
            SemanticTokenType::REGEXP,
            SemanticTokenType::OPERATOR,
            SemanticTokenType::NAMESPACE,
            SemanticTokenType::TYPE,
            SemanticTokenType::STRUCT,
            SemanticTokenType::CLASS,
            SemanticTokenType::INTERFACE,
            SemanticTokenType::ENUM,
            SemanticTokenType::ENUM_MEMBER,
            SemanticTokenType::TYPE_PARAMETER,
            SemanticTokenType::FUNCTION,
            SemanticTokenType::METHOD,
            SemanticTokenType::PROPERTY,
            SemanticTokenType::MACRO,
            SemanticTokenType::VARIABLE,
            SemanticTokenType::PARAMETER,
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
        $(pub const $ident: SemanticTokenModifier = SemanticTokenModifier::new($string);)*

        pub const SUPPORTED_MODIFIERS: &[SemanticTokenModifier] = &[
            SemanticTokenModifier::DOCUMENTATION,
            SemanticTokenModifier::DECLARATION,
            SemanticTokenModifier::DEFINITION,
            SemanticTokenModifier::STATIC,
            SemanticTokenModifier::ABSTRACT,
            SemanticTokenModifier::DEPRECATED,
            SemanticTokenModifier::READONLY,
            SemanticTokenModifier::DEFAULT_LIBRARY,
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
