def no_dups[DecidableEq α](list : List α) : Bool := list.eraseDups = list

def valid_enumeration[DecidableEq  α](list: List α) : Prop :=
  no_dups list ∧ (∀ a : α, list.contains a)

class Enumerable (α : Type)[DecidableEq  α] where
  enumeration : List α
  hvalid : valid_enumeration enumeration