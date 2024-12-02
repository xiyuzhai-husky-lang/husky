import Mathlib.Tactic.Linarith
import Mathlib.Tactic.Ring
import Mathlib.Tactic.FieldSimp
import Mathlib.Data.Real.Basic

namespace Example10

-- Let $a\in\mathbb{R}$
variable {a : ℝ}

-- Let $b\in\mathbb{R}$
variable {b : ℝ}

-- Assume $a > 0$ and $b > 0$
variable (ha : a > 0) (hb : b > 0)

-- Then $\frac{a}{b} + \frac{b}{a} - 2 = \frac{a^2 + b^2 - 2ab}{ab}$
def h (a b : ℝ) (ha : a > 0) (hb : b > 0) : a / b + b / a - 2 = (a ^ 2 + b ^ 2 - 2 * a * b) / (a * b) :=
by
  field_simp [ne_of_gt ha, ne_of_gt hb]
  ring

-- Then $\frac{a^2 + b^2 - 2ab}{ab} = \frac{{(a-b)}^2}{ab}$
def h1 (a b : ℝ) : (a ^ 2 + b ^ 2 - 2 * a * b) / (a * b) = (a - b) ^ 2 / (a * b) :=
by
  congr
  ring

-- Then $\frac{{(a-b)}^2}{ab} \ge 0$
def h2 (a b : ℝ) (ha : a > 0) (hb : b > 0) : (a - b) ^ 2 / (a * b) ≥ 0 :=
by
  have h_nonneg : (a - b) ^ 2 ≥ 0 := by nlinarith
  exact div_nonneg h_nonneg (mul_pos ha hb).le

-- Then $\frac{a}{b} + \frac{b}{a} - 2 \ge 0$
def h3 (a b : ℝ) (ha : a > 0) (hb : b > 0) : a / b + b / a - 2 ≥ 0 :=
by
  rw [h a b ha hb, h1 a b]
  exact h2 a b ha hb

-- Then $\frac{a}{b} + \frac{b}{a} \ge 2$
def h4 (a b : ℝ) (ha : a > 0) (hb : b > 0) : a / b + b / a ≥ 2 :=
by
  linarith [h3 a b ha hb]

end Example10
