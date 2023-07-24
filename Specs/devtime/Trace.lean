

structure TraceId

inductive Animal
  | Dog
    (bark : Bark)
    (height : Nat)
  | Cat
    (weight : Weight)

inductive TraceVariant
    | Main
      (repr : FeatureRepr)
    | Module
      (route : EntityRoutePtr)
      (file: FilePtr)
      (range : TextRange)
    | EntityFeature
      (route : EntityRoutePtr)
      (repr : FeatureRepr)
    | FeatureStmt
      (stmt : FeatureLazyStmt)
    | FeatureBranch
      (branch : FeatureLazyBranch)
    | FeatureExpr
      (expr : FeatureLazyExpr)
    | FeatureCallArgument 
      (name : String)
      (argument : FeatureLazyExpr)
    |  FuncStmt
      (stmt : FuncStmt)
      (history : History)
    | ProcStmt
      (stmt : ProcStmt)
      (history : History)
    | ProcBranch
      (stmt : ProcStmt)
      (branch : ProcConditionFlowBranch)
      (opt_vm_branch : Option VMConditionBranch) -- not none when executed
      (branch_idx: u8)
      (history : History)
    | FuncBranch
      (stmt : FuncStmt)
      (branch : FuncConditionFlowBranch)
      (opt_vm_branch : Option VMConditionBranch)
        -- not none when executed
      (branch_idx : u8)
      (history : History)
    | LoopFrame
      (loop_stmt : ProcStmt)
      (body_instruction_sheet: InstructionSheet)
      (body_stmts : List ProcStmt)
      (loop_frame_data: LoopFrameData)
    | EagerExpr
      (expr : EagerExpr)
      (history : History) 
    | EagerCallArgument
      (name : String)
      (argument: EagerExpr)
      (history : History)
    | CallHead
      (item : EntityDefn)
      (tokens : List TraceTokenData)