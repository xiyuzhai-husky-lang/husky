structure File

structure TextRange

structure RawStmt

inductive UseVariant

structure RawExprIdx where
  raw: Nat

mutual
inductive RawExprVariant
  | Variable
  | FrameVariable
  | ThisValue
  | ThisField
  | Unrecognized
  | Entity
  | PrimitiveLiteral
  | Bracketed
  | Opn
  | Lambda
end


namespace RawExprVariant
def subexprs (variant : RawExprVariant) : List RawExprIdx :=
  match variant with
  | Variable => []
  | FrameVariable => []
  | ThisValue => []
  | ThisField => []
  | Unrecognized => []
  | Entity => sorry
  | PrimitiveLiteral => []
  | Bracketed => sorry
  | Opn => sorry
  | Lambda => sorry
-- def height (variant : RawExprVariant) : Nat :=
--   match variant.subexprs with
--   | [] => 0
--   | _ => 0
end RawExprVariant

namespace RawExpr
def valid (range : Range) (paradigm : Paradigm) (variant: RawExprVariant) : Prop := sorry
end RawExpr

inductive Paradigm
  | LazyFunctional
  | EagerFunctional
  | EagerProcedural

structure RawExpr where
  range : Range
  paradigm : Paradigm
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
  range : Range
  variant : AstVariant