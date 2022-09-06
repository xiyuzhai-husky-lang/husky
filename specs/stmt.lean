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