import Specs.syntax.Word
import Specs.syntax.EntityRoute

structure HuskyAtom where

-- chore

inductive HuskyAtomVariant
    | EntityRoute (route: EntityRoutePtr) (kind: EntityKind)
    | Variable
        (varname: CustomIdentifier)
        (init_range: TextRange)
    | FrameVariable
        (varname: CustomIdentifier)
        (init_range: TextRange)
    | ThisValue
        (opt_this_ty: Option EntityRoutePtr)
        (opt_this_liason: Option ParameterModifier)
    | ThisField
        (field_ident: RangedCustomIdentifier)
        (opt_this_ty: Option EntityRoutePtr)
        (opt_this_liason: Option ParameterModifier)
        (opt_field_ty: Option RangedEntityRoute)
        (field_liason: MemberModifier)
    | Unrecognized (ident : CustomIdentifier)
    | PrimitiveLiteral(data : RawLiteralData)
    | Binary(opr : BinaryOpr)
    | Prefix(opr : PrefixOpr)
    | Suffix(opr : RawSuffixOpr)
    | FieldAccess(ident : RangedCustomIdentifier)
    | ListStart (bra : Bracket) (attr : ListStartAttr)
    | ListEnd (ket : Bracket) (attr : ListEndAttr)
    | ListItem
    | LambdaHead (parameters : List RangedCustomIdentifier × (Option RangedEntityRoute))
    | SilentEnd
    | Be
    | BePattern (patt : RawPattern)
    | WordPattern (patt : WordPattern)