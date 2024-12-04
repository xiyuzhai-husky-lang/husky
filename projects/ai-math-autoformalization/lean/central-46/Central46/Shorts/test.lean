import Mathlib.Tactic.Linarith
import Mathlib.Tactic.Ring
import Mathlib.Data.Real.Basic
import Mathlib.Data.Real.Sqrt
import Mathlib.Tactic.Explode

namespace Example

-- Let $x\in\mathbb{R}$
variable (x : ℝ)

-- Assume $x>0$
variable (h : x > 0)

-- Then $x + \frac{1}{x} - 2 = \frac{x^2 + 1 - 2x}{x}$
def h1 : x + 1 / x - 2 = (x ^ 2 + 1 - 2 * x) / x :=
by
  have key : x ≠ 0 := by linarith [h]
  field_simp
  ring

-- Then $\frac{x^2 + 1 - 2x}{x} = \frac{{(x-1)}^2}{x}$
def h2 : (x ^ 2 + 1 - 2 * x) / x = (x - 1) ^ 2 / x :=
by
  have key : x ≠ 0 := by linarith [h]
  field_simp
  ring

-- Then $x + \frac{1}{x} - 2 = \frac{{(x-1)}^2}{x}$
def h3 : x + 1 / x - 2 = (x - 1) ^ 2 / x :=
by
  rw [h1 x h]
  exact h2 x h

-- Then $\frac{{(x-1)}^2}{x} \ge 0$
def h4 : (x - 1) ^ 2 / x ≥ 0 :=
by
  have key : x > 0 := h
  have key2 : (x - 1) ^ 2 ≥ 0 := by apply sq_nonneg
  nlinarith

-- Then $x + \frac{1}{x} - 2 \ge 0$
def h5 : x + 1 / x - 2 ≥ 0 :=
by
  rw [h3 x h]
  exact h4 x h

-- Then $x + \frac{1}{x} \ge 2$
def h6 : x + 1 / x ≥ 2 :=
by
  linarith [h5 x h]
end Example
