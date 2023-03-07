import Specs.abstraction

inductive RefQualifier
  | None
  | Leash
  | TempRef
  | TempRefMut
  deriving DecidableEq

namespace RefQualifier
def toRustVersion : RefQualifier -> String
  | None => "RefQualifier::None"
  | Leash => "RefQualifier::Leash"
  | TempRef => "RefQualifier::TempRef"
  | TempRefMut => "RefQualifier::TempRefMut"

instance : Enumerable RefQualifier where
  enumeration := [None, Leash, TempRef, TempRefMut]
  hvalid := by
    apply And.intro
    apply rfl
    intro a
    cases a <;> rfl

instance : Concept RefQualifier where
  description : RefQualifier -> String
  | None => sorry
  | Leash => sorry
  | TempRef => sorry
  | TempRefMut => sorry
  descriptions := "haha"
end RefQualifier

def descriptions : List String := []