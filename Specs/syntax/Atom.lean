import Specs.syntax.Word
import Specs.syntax.EntityRoute

structure HuskyAtom where

-- chore

inductive HuskyAtomVariant
    | EntityRoute (route: EntityRoutePtr) (kind: EntityKind)
    | Variable
        (varname: Identifier)
        (init_range: TextRange)
    | FrameVariable
        (varname: Identifier)
        (init_range: TextRange)
    | ThisValue
        (opt_this_ty: Option EntityRoutePtr)
        (opt_this_liason: Option ParameterModifier)
    | ThisField
        (field_ident: RangedIdentifier)
        (opt_this_ty: Option EntityRoutePtr)
        (opt_this_liason: Option ParameterModifier)
        (opt_field_ty: Option Term)
        (field_liason: MemberModifier)
    | Unrecognized (ident : Identifier)
    | PrimitiveLiteral(data : LiteralToken)
    | Binary(opr : BinaryOpr)
    | Prefix(opr : PrefixOpr)
    | Suffix(opr : RawSuffixOpr)
    | FieldAccess(ident : RangedIdentifier)
    | ListStart (bra : Bracket) (attr : ListStartAttr)
    | ListEnd (ket : Bracket) (attr : ListEndAttr)
    | ListItem
    | LambdaHead (parameters : List RangedIdentifier × (Option Term))
    | SilentEnd
    | Be
    | BePattern (patt : RawPattern)
    | WordPattern (patt : WordPattern)