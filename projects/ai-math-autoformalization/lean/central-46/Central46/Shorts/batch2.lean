import Mathlib.Tactic.Linarith
import Mathlib.Tactic.Ring
import Mathlib.Data.Real.Basic
import Mathlib.Data.Real.Sqrt
import Mathlib.Tactic.Explode

namespace Example1
-- Let $x\in\mathbb{R}$
variable (x : ℝ)

-- Let $y\in\mathbb{R}$
variable (y : ℝ)

-- Then $4 x^2 + \frac{y^2}{4} - 2 x y = {(2 x)}^2 + {(\frac{y}{2})}^2 - 2 (2 x) (\frac{y}{2})$
def h : 4 * (x ^ 2) + y ^ 2 / 4 - 2 * x * y = 2 * x ^ 2 + y / 2 ^ 2 - 2 * (2 * x) * (y / 2) := sorry

-- Then ${(2 x)}^2 + {(\frac{y}{2})}^2 - 2 (2 x) (\frac{y}{2}) = {(2 x - \frac{y}{2})}^2$
def h1 : 2 * x ^ 2 + y / 2 ^ 2 - 2 * (2 * x) * (y / 2) = (2 * x - y / 2) ^ 2 := sorry

-- Then ${(2 x - \frac{y}{2})}^2 \ge 0$
def h2 : (2 * x - y / 2) ^ 2 ≥ 0 := sorry

-- Then $4 x^2 + \frac{y^2}{4} - 2 x y \ge 0$
def h3 : 4 * (x ^ 2) + y ^ 2 / 4 - 2 * x * y ≥ 0 := sorry

-- Then $4 x^2 + \frac{y^2}{4} \ge 2 x y$
def h4 : 4 * (x ^ 2) + y ^ 2 / 4 ≥ 2 * x * y := sorry
end Example1
