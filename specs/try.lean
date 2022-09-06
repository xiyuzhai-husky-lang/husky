import Init.Data.Basic
import Init.Prelude

structure A where
  (a : Nat)
  (b : a > 0)

def c : A := { a := 1 , b := by simp }

def a := List Prop