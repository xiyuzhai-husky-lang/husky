import Mathlib.Tactic.Linarith
import Mathlib.Tactic.Ring
import Mathlib.Data.Real.Basic

namespace Example7

-- Let $x\in\mathbb{R}$
variable (x : ℝ)

-- Then $x^2 + 1 - 2x = {(x-1)}^2$
def h : x ^ 2 + 1 - 2 * x = (x - 1) ^ 2 :=
by
  ring

-- Then ${(x-1)}^2 \ge 0$
def h1 : (x - 1) ^ 2 ≥ 0 :=
by
  nlinarith

-- Then $x^2 + 1 - 2x \ge 0$
def h2 : x ^ 2 + 1 - 2 * x ≥ 0 :=
by
  rw [h x]
  exact h1 x

-- Then $x^2 + 1 \ge 2x$
def h3 : x ^ 2 + 1 ≥ 2 * x :=
by
  linarith [h2 x]

end Example7
