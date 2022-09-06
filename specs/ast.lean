structure File

structure Range

structure RawStmt

inductive UseVariant

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

namespace RawExprVariant
def subexprs (variant : RawExprVariant) : List RawExpr :=
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
  hvalid: RawExpr.valid range paradigm variant


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