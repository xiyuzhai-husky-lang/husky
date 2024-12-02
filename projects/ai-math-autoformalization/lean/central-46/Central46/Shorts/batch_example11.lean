import Mathlib.Tactic.Linarith
import Mathlib.Tactic.Ring
import Mathlib.Data.Real.Basic

namespace Example11

-- Let $x\in\mathbb{R}$
variable (x : ℝ)

-- Let $y\in\mathbb{R}$
variable (y : ℝ)

-- Then $x^2 + y^2 - 2xy = {(x-y)}^2$
def h : x ^ 2 + y ^ 2 - 2 * x * y = (x - y) ^ 2 :=
by
  ring

-- Then ${(x-y)}^2 \ge 0$
def h1 : (x - y) ^ 2 ≥ 0 :=
by
  nlinarith

-- Then $x^2 + y^2 - 2xy \ge 0$
def h2 : x ^ 2 + y ^ 2 - 2 * x * y ≥ 0 :=
by
  rw [h x y]
  exact h1 x y

-- Then $x^2 + y^2 \ge 2xy$
def h3 : x ^ 2 + y ^ 2 ≥ 2 * x * y :=
by
  linarith [h2 x y]

end Example11
