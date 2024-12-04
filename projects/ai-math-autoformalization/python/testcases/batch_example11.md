Problem:
For $x, y \in \mathbb{R}$, prove that $x^2 + y^2 \ge 2 xy$.

LaTeX Proof:
Let $x\in\mathbb{R}$. Let $y\in\mathbb{R}$. Then $x^2 + y^2 - 2xy = {(x-y)}^2$. Then ${(x-y)}^2 \ge 0$. Then $x^2 + y^2 - 2xy \ge 0$. Then $x^2 + y^2 \ge 2xy$.

Lean Proof:
``` Lean
import Mathlib.Tactic.Linarith
import Mathlib.Tactic.Ring
import Mathlib.Data.Real.Basic
import Mathlib.Data.Real.Sqrt
import Mathlib.Tactic.Explode

namespace Example
-- Let $x\in\mathbb{R}$
variable (x : ℝ)

-- Let $y\in\mathbb{R}$
variable (y : ℝ)

-- Then $x^2 + y^2 - 2xy = {(x-y)}^2$
def h : x ^ 2 + y ^ 2 - 2 * x * y = (x - y) ^ 2 := sorry

-- Then ${(x-y)}^2 \ge 0$
def h1 : (x - y) ^ 2 ≥ 0 := sorry

-- Then $x^2 + y^2 - 2xy \ge 0$
def h2 : x ^ 2 + y ^ 2 - 2 * x * y ≥ 0 := sorry

-- Then $x^2 + y^2 \ge 2xy$
def h3 : x ^ 2 + y ^ 2 ≥ 2 * x * y := sorry
end Example
```