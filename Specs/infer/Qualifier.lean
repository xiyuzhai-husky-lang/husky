import Specs.abstraction

inductive RefQualifier
  | None
  | EvalRef
  | TempRef
  | TempRefMut
  deriving DecidableEq

namespace RefQualifier
def toRustVersion : RefQualifier -> String
  | None => "RefQualifier::None"
  | EvalRef => "RefQualifier::EvalRef"
  | TempRef => "RefQualifier::TempRef"
  | TempRefMut => "RefQualifier::TempRefMut"

instance : Enumerable RefQualifier where
  enumeration := [None, EvalRef, TempRef, TempRefMut]
  hvalid := by
    apply And.intro
    apply rfl
    intro a
    cases a <;> rfl

instance : Concept RefQualifier where
  description : RefQualifier -> String
  | None => sorry
  | EvalRef => sorry
  | TempRef => sorry
  | TempRefMut => sorry
  descriptions := "haha"
end RefQualifier

def descriptions : List String := []