import Mathlib.Tactic.Linarith
import Mathlib.Tactic.Ring
import Mathlib.Data.Real.Basic
import Mathlib.Tactic.Explode

namespace Example1
def h : 1 + 1 = 2 := sorry
end Example1

namespace Example2
variable (x : ℝ)

def h1 : x ^ 2 ≥ 0 := sorry
end Example2

namespace Example3
variable (a : ℝ)

variable (b : ℝ)

def h2 : a + b = b + a := sorry
end Example3

namespace Example4
variable (x1 : ℝ)

def h3 : x1 ^ 2 ≥ 0 := sorry
end Example4

namespace Example5
variable (x2 : ℝ)

variable (h4 : x2 > 0)

def h5 : x2 + 1 / x2 - 2 = (x2 ^ 2 + 1 - 2 * x2) / x2 := sorry

def h6 : (x2 ^ 2 + 1 - 2 * x2) / x2 = (x2 - 1) ^ 2 / x2 := sorry

def h7 : x2 + 1 / x2 - 2 = (x2 - 1) ^ 2 / x2 := sorry

def h8 : (x2 - 1) ^ 2 / x2 ≥ 0 := sorry

def h9 : x2 + 1 / x2 - 2 ≥ 0 := sorry

def h10 : x2 + 1 / x2 ≥ 2 := sorry
end Example5

namespace Example6
variable (x3 : ℝ)

def h11 : x3 + 1 / x3 - 2 = (x3 ^ 2 + 1 - 2 * x3) / x3 := sorry

def h12 : (x3 ^ 2 + 1 - 2 * x3) / x3 = (x3 - 1) ^ 2 / x3 := sorry

def h13 : x3 + 1 / x3 - 2 = (x3 - 1) ^ 2 / x3 := sorry

def h14 : (x3 - 1) ^ 2 / x3 ≥ 0 := sorry

def h15 : x3 + 1 / x3 - 2 ≥ 0 := sorry

def h16 : x3 + 1 / x3 ≥ 2 := sorry
end Example6
