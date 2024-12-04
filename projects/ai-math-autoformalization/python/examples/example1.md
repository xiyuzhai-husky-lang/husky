Problem:
For any $x \in \mathbb{R}$, prove that $x^2 + 1 \ge 2 x$.

LaTex Proof:
Let $x\in\mathbb{R}$. Then $x^2 + 1 - 2 x = {(x - 1)}^2$. Then ${(x - 1)}^2 \ge 0$. Then $x^2 + 1 \ge 2 x$.

Lean Proof:
``` Lean
import Mathlib.Tactic.Ring
import Mathlib.Data.Real.Basic

namespace Example

-- Let $x\in\mathbb{R}$.
variable {x : ℝ}

-- Then $x^2 + 1 - 2 x = {(x - 1)}^2$.
def h1 (x : ℝ) : x ^ 2 + 1 - 2 * x = (x - 1) ^ 2 :=
by
  ring

-- Then ${(x - 1)}^2 \ge 0$.
def h2 (x : ℝ) : (x - 1) ^ 2 ≥ 0 :=
by
  apply sq_nonneg

-- Then $x^2 + 1 \ge 2 x$.
def h3 (x : ℝ) : x ^ 2 + 1 ≥ 2 * x :=
by
  have key : x ^ 2 + 1 - 2 * x ≥ 0 := by
    rw [h1 x]
    exact h2 x
  exact sub_nonneg.mp key

end Example
```
