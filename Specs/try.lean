import Init.Data.Basic
import Init.Prelude

-- structure A where
--   (a : Nat)
--   (b : a > 0)

-- def c : A := { a := 1 , b := by simp }

-- namespace A
-- def haha (self : A) : Nat := 1
-- end A

inductive TYPE_NAME
  | VARIANT1
  | VARIANT2

inductive Animal
  | Dog
  | Cat



inductive Animal2
  | Dog (weight : Nat)
  | Cat (height : Nat) (weight : Nat)