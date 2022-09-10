def no_dups[BEq α](list : List α) : Bool := list.eraseDups == list

def valid_enumeration[BEq  α](list: List α) : Prop :=
  no_dups list ∧ (∀ a : α, list.contains a)

structure Enumerable (α : Type)[BEq  α] where
  enumeration : List α
  hvalid : valid_enumeration enumeration