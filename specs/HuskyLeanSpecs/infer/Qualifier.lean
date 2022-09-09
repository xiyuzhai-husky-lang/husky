import HuskyLeanSpecs.abstraction.Enumerable

inductive RefQualifier
  | None
  | EvalRef
  | TempRef
  | TempRefMut

deriving instance BEq for RefQualifier
deriving instance DecidableEq for RefQualifier

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