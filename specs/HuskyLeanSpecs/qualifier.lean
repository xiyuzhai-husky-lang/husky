inductive RefQualifier
  | None
  | EvalRef
  | TempRef
  | TempRefMut

namespace RefQualifier
instance : BEq RefQualifier where
  beq
    | None, None => true
    | EvalRef, EvalRef => true
    | TempRef, TempRef => true
    | TempRefMut, TempRefMut => true
    | _, _ => false
end RefQualifier

def all_ref_qualifiers : List RefQualifier :=
  [RefQualifier.None, RefQualifier.EvalRef, RefQualifier.TempRef, RefQualifier.TempRefMut]

def no_dups[BEq α](list : List α) : Bool := list.eraseDups == list

def is_enumeration[BEq α](list: List α) : Prop :=
  no_dups list ∧ (∀ a : α, list.contains a)

namespace RefQualifier
example : is_enumeration all_ref_qualifiers := by
  apply And.intro
  apply rfl
  intro a
  cases a with
  | None => rfl
  | EvalRef => rfl
  | TempRef => rfl
  | TempRefMut => rfl
end RefQualifier
