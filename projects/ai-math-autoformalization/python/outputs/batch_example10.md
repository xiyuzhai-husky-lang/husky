Here is the completed Lean proof:

``` Lean
import Mathlib.Tactic.Linarith
import Mathlib.Tactic.Ring
import Mathlib.Data.Real.Basic
import Mathlib.Data.Real.Sqrt
import Mathlib.Tactic.Explode

namespace Example

-- Let $a\in\mathbb{R}$
variable (a : ℝ)

-- Assume $a > 0$
variable (h : a > 0)

-- Let $b\in\mathbb{R}$
variable (b : ℝ)

-- Assume $b > 0$
variable (h1 : b > 0)

-- Then $\frac{a}{b} + \frac{b}{a} - 2 = \frac{a^2 + b^2 - 2ab}{ab}$
def h2 : a / b + b / a - 2 = (a ^ 2 + b ^ 2 - 2 * a * b) / (a * b) :=
by
  field_simp
  ring

-- Then $\frac{a^2 + b^2 - 2ab}{ab} = \frac{{(a-b)}^2}{ab}$
def h3 : (a ^ 2 + b ^ 2 - 2 * a * b) / (a * b) = (a - b) ^ 2 / (a * b) :=
by
  ring

-- Then $\frac{{(a-b)}^2}{ab} \ge 0$
def h4 : (a - b) ^ 2 / (a * b) ≥ 0 :=
by
  have h2 : a * b > 0 := by linarith
  have h3 : (a - b) ^ 2 ≥ 0 := by apply sq_nonneg
  exact div_nonneg h3 h2

-- Then $\frac{a}{b} + \frac{b}{a} - 2 \ge 0$
def h5 : a / b + b / a - 2 ≥ 0 :=
by
  rw [h2 a h b h1]
  exact h4 a h b h1

-- Then $\frac{a}{b} + \frac{b}{a} \ge 2$
def h6 : a / b + b / a ≥ 2 :=
by
  linarith [h5 a h b h1]
end Example
```

Note that I used `field_simp` and `ring` tactics to simplify the expressions, and `linarith` to prove the inequalities. I also used `sq_nonneg` to prove that a square is non-negative, and `div_nonneg` to prove that a division is non-negative.