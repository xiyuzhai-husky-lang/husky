import HuskyLeanSpecs.abstraction.Enumerable

inductive RefQualifier
  | None
  | EvalRef
  | TempRef
  | TempRefMut
  deriving BEq

namespace RefQualifier
instance : Enumerable RefQualifier where
  enumeration := [None, EvalRef, TempRef, TempRefMut]
  hvalid := by
    apply And.intro
    apply rfl
    intro a
    cases a with
    | _ => rfl
end RefQualifier