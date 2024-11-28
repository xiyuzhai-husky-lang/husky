import Mathlib.Tactic.Linarith
import Mathlib.Tactic.Ring
import Mathlib.Data.Real.Basic
import Mathlib.Tactic.Explode

namespace Example1
-- Then $1+1=2$
def h : 1 + 1 = 2 := sorry
end Example1

namespace Example2
-- Let $x\in\mathbb{R}$
variable (x : ℝ)

-- Then $x^2\ge 0$
def h : x ^ 2 ≥ 0 := sorry
end Example2

namespace Example3
-- Let $x\in\mathbb{R}$
variable (x : ℝ)

-- Then $x^2 + 1\ge 2x$
def h : x ^ 2 + 1 ≥ 2 * x := sorry
end Example3

namespace Example4
-- Then $1=1=1$
def h : 1 = 1 := by
  calc
    1 = 1 := sorry
    _ = 1 := sorry

-- Then $1=1<2$
def h1 : 1 < 2 := by
  calc
    1 = 1 := sorry
    _ < 2 := sorry

-- Then $1.0 \le 1 \le 1$
def h2 : 1.0 ≤ 1 := by
  calc
    1.0 ≤ 1 := sorry
    _ ≤ 1 := sorry
end Example4
