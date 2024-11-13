import Mathlib.Tactic.Linarith
import Mathlib.Tactic.Ring

def hello := "world"

#eval 1

def f (x : Nat) := x + 1

example : 1 + 1 = 2:= by simp

section example1
variable (a b c : Nat)
example : a + b + c = a + c + b := sorry
def x := a + b + c
def y := x
def d := b
def e := x 1 2 3
#check x
#check y

#check f (x a b c)
end example1

#check x

def z := x
#check z

-- nested sections are allowed
section layer1
section layer2
end layer2
end layer1

variable (x: Int)

example : x^2 + 1 >= 2 * x := by
  have h : x^2 - 2*x + 1 = (x-1)^2 := by ring
  have h2 : 0 ≤ (x-1)^2 := sq_nonneg (x-1)
  rw [←h] at h2
  linarith
