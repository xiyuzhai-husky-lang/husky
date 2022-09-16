import Specs.syntax

inductive EagerExpr
  | Variable ( varname : String )
  | ThisValue
  | ThisField ( field_ident: String )
  | PrimitiveLiteral (data : PrimitiveLiteralData)
  | EnumKindLiteral (route : EntityRoute)
  | Bracketed ( expr : EagerExpr )
  | Opn (opds : List EagerExpr)
  | Lambda
  | EntityThickFp
  | EntityFeature

instance : DecidableEq EagerExpr := by sorry


inductive Expr
  | Variable
  | Opn (opds : List Expr)

instance Expr.deq : DecidableEq Expr
| Variable, Variable => isTrue rfl
| Variable, Opn _ => isFalse (by simp)
| Opn _, Variable => isFalse (by simp)
| Opn [], Opn [] => isTrue rfl
| Opn (_ :: _), Opn [] => isFalse (by simp)
| Opn [], Opn (_ :: _) => isFalse (by simp)
| Opn (x :: xs), Opn (y :: ys) =>
  have := Expr.deq x y
  if hx : x = y then
    match Expr.deq (Opn xs) (Opn ys) with
    | isTrue h => isTrue (by { simp at h; simp [hx, h] })
    | isFalse h => isFalse (by { intro h'; apply h; simp at h'; simp [h'.2] })
  else isFalse (by simp [hx])


-- def Expr.deq (a b : Expr) : Decidable (a = b) := sorry
-- -- instance : DecidableEq Expr := by sorry

structure ExprWrapper where
  expr : Expr
  deriving DecidableEq


structure LazyStmt

inductive LoopVariant

namespace LoopVariant
def subexprs : LoopVariant -> List Expr
  | Variable ( varname : String )
  | ThisValue
  | ThisField ( field_ident: String )
  | PrimitiveLiteral _ => []
  | EnumKindLiteral _ => []
  | Bracketed expr => [expr]
  | Opn opds => opds
  | Lambda
  | EntityThickFp
  | EntityFeature
end LoopVariant

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