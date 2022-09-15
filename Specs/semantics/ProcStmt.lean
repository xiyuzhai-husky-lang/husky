import Specs.syntax.EntityRoute

inductive PrimitiveLiteralData
  deriving DecidableEq

inductive EagerExpr
  | Variable ( varname : String )
  | ThisValue
  | ThisField ( field_ident: String )
  | PrimitiveLiteral : PrimitiveLiteralData -> EagerExpr
  | EnumKindLiteral : EntityRoute -> EagerExpr
  | Bracketed ( expr : EagerExpr )
  | Opn
  | Lambda
  | EntityThickFp
  | EntityFeature
  deriving DecidableEq

structure LazyStmt

inductive LoopVariant

namespace LoopVariant
def subexprs : LoopVariant -> List Expr := sorry
end LoopVariant

structure CustomIdentifier

structure File
structure Range

mutual
inductive ProcConditionFlowBranch where
  | _ (variant: ProcConditionFlowBranchVariant) (stmts: List ProcStmt) : ProcConditionFlowBranch

inductive ProcConditionFlowBranchVariant
  | If (condition : EagerExpr) : ProcConditionFlowBranchVariant
  | Elif (condition : EagerExpr) : ProcConditionFlowBranchVariant
  | Else

inductive ProcStmt where
  | _ (file : File) (range : Range) (variant : ProcStmtVariant)

inductive ProcStmtVariant where
  | Init (varname : CustomIdentifier) (initial_value : EagerExpr)
  | Assert (condition : EagerExpr)
  | Execute (expr : EagerExpr)
  | ConditionFlow (branches : List ProcConditionFlowBranch)
  | Loop (loop_variant : LoopVariant) (stmts : List ProcStmt)
  | Break
  | Return (result : EagerExpr)
  | Match (match_expr : EagerExpr)
end
namespace ProcStmtVariant
def subexprs : ProcStmtVariant -> List EagerExpr
  | Init (_) (initial_value) => [initial_value]
  | Assert (condition) => [condition]
  | Execute (expr) => [expr]
  | ConditionFlow (_) => []
  | Loop (loop_variant : LoopVariant) (_) => loop_variant.subexprs
  | Break => []
  | Return (result : EagerExpr) => [result]
  | Match (match_expr : EagerExpr) => [match_expr]
end ProcStmtVariant

mutual
inductive FuncConditionFlowBranch where
  | _ (variant: FuncConditionFlowBranchVariant) (stmts: List FuncStmt) : FuncConditionFlowBranch

inductive FuncConditionFlowBranchVariant
  | If (condition : EagerExpr) : FuncConditionFlowBranchVariant
  | Elif (condition : EagerExpr) : FuncConditionFlowBranchVariant
  | Else

inductive FuncStmt where
  | _ (file : File) (range : Range) (variant : FuncStmtVariant)

inductive FuncStmtVariant where
  | Init (varname : CustomIdentifier) (initial_value : EagerExpr)
  | Assert (condition : EagerExpr)
  | ConditionFlow (branches : List FuncConditionFlowBranch)
  | Return (result : EagerExpr)
  | Match (match_expr : EagerExpr)
end