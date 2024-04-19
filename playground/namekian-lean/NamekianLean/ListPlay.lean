def as: List Nat := [1, 3]

def bs: List Nat := [2, 4]

namespace List
def flatten(a_mat: List $ List α): List α :=
  List.foldl (fun a b => a.append b) [] a_mat
end List

instance: Monad List where
  pure a := [a]
  bind as f:= (as.map f).flatten

def cs: List (Nat × Nat) := do
   let a <- as
   let b <- bs
   pure (a,b)

#eval cs

def print_ty(a: Type) : String :=
  let a := s!"a = {1}"
  sorry
