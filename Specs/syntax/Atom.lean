import Specs.syntax

structure HuskyAtom where

-- chore

inductive HuskyAtomVariant
    | EntityRoute (route: EntityRoutePtr) (kind: EntityKind)
    Variable {
        varname: CustomIdentifier,
        init_range: TextRange,
    },
    FrameVariable {
        varname: CustomIdentifier,
        init_range: TextRange,
    },
    ThisValue {
        opt_this_ty: Option<EntityRoutePtr>,
        opt_this_liason: Option<ParameterModifier>,
    },
    ThisField {
        field_ident: RangedCustomIdentifier,
        opt_this_ty: Option<EntityRoutePtr>,
        opt_this_liason: Option<ParameterModifier>,
        opt_field_ty: Option<RangedEntityRoute>,
        field_liason: MemberModifier,
    },
    Unrecognized(CustomIdentifier),
    PrimitiveLiteral(PrimitiveLiteralData),
    Binary(BinaryOpr),
    Prefix(PrefixOpr),
    Suffix(RawSuffixOpr),
    FieldAccess(RangedCustomIdentifier),
    ListStart(Bracket, ListStartAttr),
    ListEnd(Bracket, ListEndAttr),
    ListItem,
    LambdaHead(Vec<(RangedCustomIdentifier, Option<RangedEntityRoute>)>),
    SilentEnd,
    Be,
    BePattern(RawPattern),
    WordPattern {
        patt: WordPattern,
    },