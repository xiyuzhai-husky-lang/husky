inductive Binding
| DerefCopy
| DerefMove
| DerefPure
| RefMut
| RefConst
| Move
| Copy
| Take

structure StackIdx where
  raw : USize

structure Qualifier where
  mutable : Bool
  opt_stack_idx : Option StackIdx