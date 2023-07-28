import Specs.vm.Register
import Specs.vm.Instruction

structure VMStack where
  values : List __RegularValue

namespace VMStack
open InstructionData
def exec (ins: Instruction) : VMStack :=
  match ins.variant with
  | PushVariable
    stack_idx
    binding
    range
    ty 
    varname
    explicit => sorry
  | PushLiteralValue 
    value
    ty
    explicit => sorry
  | CallRoutine 
    resolved_linkage 
    nargs
    return_ty
    discard => sorry
  | CallInterpreted 
    routine_uid
    nargs
    return_ty
    discard   => sorry
  | VirtualStructField
    field_idx
    field_binding
    field_ty    => sorry
  | NewVirtualStruct 
    ty
    fields  => sorry  
  | Loop
    body 
    loop_kind  => sorry
  | Return 
    return_ty => sorry
  | BreakIfFalse => sorry
  | Break => sorry
  | Assert => sorry
  | Require => sorry
  | ConditionFlow 
    branches => sorry
  | PatternMatch 
    branches  => sorry
  | EntityFeature
    feature_uid
    ty  => sorry
  | PushEntityFp
    opt_linkage
    ty
    opt_instruction_sheet => sorry
end VMStack