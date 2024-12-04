import Mathlib.Tactic.Linarith
import Mathlib.Tactic.Ring
import Mathlib.Data.Real.Basic
import Mathlib.Data.Real.Sqrt
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

-- Variable declaration for real numbers
variable {x : ℝ}

-- Step 1: Rewrite the expression as a square
def h1 (x : ℝ) : x ^ 2 + 1 - 2 * x = (x - 1) ^ 2 :=
by
  ring

-- Step 2: State the non-negativity of the square
def h2 (x : ℝ) : (x - 1) ^ 2 ≥ 0 :=
by
  apply sq_nonneg

-- Step 3: Conclude the inequality
def h3 (x : ℝ) : x ^ 2 + 1 ≥ 2 * x :=
by
  rw [←sub_nonneg, h1 x]
  exact h2 x

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
