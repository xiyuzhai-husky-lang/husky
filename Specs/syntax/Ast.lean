import Specs.syntax.Word
import Specs.syntax.EntityRoute


structure RawStmt

inductive UseVariant

structure RawExprIdx where
  raw: Nat

structure RawExprRange where
  start : RawExprIdx
  end_ : RawExprIdx

namespace RawExprRange
def to_list (self : RawExprRange) : List RawExprIdx := sorry
end RawExprRange

structure ParameterModifier

inductive RawOpnVariant

mutual
inductive RawExprVariant
  | Variable (varname : Ident) (init_range : TextRange)
  | FrameVariable (varname : Ident) (init_range : TextRange)
  | ThisValue
  | ThisField
    (opt_this_ty : Option EntityRoute)
    (opt_this_liason : Option ParameterModifier)
    (field_ident: RangedIdent)
  | Unrecognized
  | Entity
  | PrimitiveLiteral
  | Bracketed
  | Opn
    (opn_variant: RawOpnVariant)
    (opds : RawExprRange)
  | Lambda
end


namespace RawExprVariant
def subexprs (variant : RawExprVariant) : List RawExprIdx :=
  match variant with
  | Variable (varname) (init_range) => []
  | FrameVariable (varname) (init_range) => []
  | ThisValue => []
  | ThisField (opt_this_ty) (opt_this_liason) (field_ident)=> []
  | Unrecognized => []
  | Entity => []
  | PrimitiveLiteral => []
  | Bracketed => sorry
  | Opn (opn_variant) (opds)=> opds.to_list
  | Lambda => sorry
end RawExprVariant

namespace RawExpr
def valid (range : Range) (paradigm : Paradigm) (variant: RawExprVariant) : Prop := sorry
end RawExpr

structure RawExpr where
  range : TextRange
  variant : RawExprVariant


inductive AstVariant where
  | TypeDefnHead
  | MainDefnHead
  | CallFormDefnHead
  | FeatureDefnHead
  | FieldDefnHead
  | DatasetConfigDefnHead
  | Stmt (stmt : RawStmt) : AstVariant
  | Use (use_variant : UseVariant) : AstVariant
  | Submodule
  | Visual

structure Ast where
  range : TextRange
  variant : AstVariant