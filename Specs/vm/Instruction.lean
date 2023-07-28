inductive InstructionData
  | PushVariable
    (stack_idx : VMStackIdx)
    (binding : Binding)
    (range : TextRange)
    (ty : EntityRoutePtr)
    (varname : Ident)
    (explicit : Bool)  
  | PushLiteralValue 
    (value : __RegularValue)
    (ty : EntityRoutePtr)
    (explicit : Bool)
  | CallRoutine 
    (resolved_linkage : __ResolvedLinkage)
    (nargs : u8)
    (return_ty : EntityRoutePtr)
    (discard : Bool)   
  | CallInterpreted 
    (routine_uid : EntityUid)
    (nargs : u8)
    (return_ty : EntityRoutePtr)
    (discard : Bool)   
  | VirtualStructField
    (field_idx : u8)
    (field_binding : Binding)
    (field_ty : EntityRoutePtr)    
  | NewVirtualStruct 
    (ty : EntityRoutePtr)
    (fields : List Ident)    
  | Loop
    (body : Instructions)
    (loop_kind : VMLoopKind)
  | Return 
    (return_ty: EntityRoutePtr)
  | BreakIfFalse
  | Break
  | Assert
  | Require
  | ConditionFlow 
    (branches : List VMConditionBranch)
  | PatternMatch 
    (branches : List VMPatternBranch)
  | EntityFeature
    (feature_uid : EntityUid)
    (ty : EntityRoutePtr) 
  | PushEntityFp
    (opt_linkage: Option __Linkage)
    (ty: EntityRoutePtr)
    (opt_instruction_sheet: Option Instructions)

structure Instruction where
    variant : InstructionData
