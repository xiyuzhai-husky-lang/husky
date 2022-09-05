inductive PrimitiveLiteralData

inductive EntityRouteVariant
  | Root : RootIdentifier -> EntityRouteVariant

structure EntityRoute where
  variant : EntityRouteVariant

inductive EagerExpr
  | Variable ( varname : String ) : EagerExpr
  | ThisValue
  | ThisField ( field_ident: String ) : EagerExpr
  | PrimitiveLiteral : PrimitiveLiteralData -> EagerExpr
  | EnumKindLiteral : EntityRoute -> EagerExpr
  | Bracketed ( expr : EagerExpr ) : EagerExpr
  | Opn
  | Lambda
  | EntityThickFp
  | EntityFeature

structure LazyStmt

inductive LoopVariant

structure CustomIdentifier

inductive ConditionFlowBranchVariant ( Expr : Type )
  | If (condition : Expr) : ConditionFlowBranchVariant Expr
  | Elif (condition : Expr) : ConditionFlowBranchVariant Expr
  | Else

structure ConditionFlowBranch ( Expr : Type ) ( Stmt : Type ) where
  variant: ConditionFlowBranchVariant Expr
  stmts: List Stmt

inductive ProcStmt where
  | Init (varname : CustomIdentifier) (initial_value : EagerExpr) : ProcStmt
  | Assert (condition : EagerExpr) : ProcStmt
  | Execute (expr : EagerExpr) : ProcStmt
  | ConditionFlow (branches : List (ConditionFlowBranch EagerExpr ProcStmt)) : ProcStmt
  | Loop (loop_variant : LoopVariant) (stmts : List ProcStmt) : ProcStmt
  | Break
  | Return (result : EagerExpr) : ProcStmt
  | Match (match_expr : EagerExpr) : ProcStmt

inductive FuncStmt where
  | Init (varname : CustomIdentifier) (initial_value : EagerExpr) : FuncStmt
  | Assert (condition : EagerExpr) : FuncStmt
  | ConditionFlow (branches : List (ConditionFlowBranch EagerExpr FuncStmt)) : FuncStmt
  | Return (result : EagerExpr) : FuncStmt
  | Match (match_expr : EagerExpr) : FuncStmt