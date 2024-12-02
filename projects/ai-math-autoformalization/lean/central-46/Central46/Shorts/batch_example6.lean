-- import Mathlib.Tactic.Linarith
-- import Mathlib.Tactic.Ring
-- import Mathlib.Tactic.FieldSimp
-- import Mathlib.Data.Real.Basic

-- namespace Example6

-- -- Let $x\in\mathbb{R}$
-- variable {x : ℝ}

-- -- Assume $x > 0$
-- variable (h : x > 0)

-- -- Then $x + \frac{1}{x} - 2 = \frac{x^2 + 1 - 2x}{x}$
-- def h (x : ℝ) (h : x > 0) : x + 1 / x - 2 = (x ^ 2 + 1 - 2 * x) / x :=
-- by
--   field_simp [ne_of_gt h]
--   ring

-- -- Then $\frac{x^2 + 1 - 2x}{x} = \frac{{(x-1)}^2}{x}$
-- def h1 (x : ℝ) : (x ^ 2 + 1 - 2 * x) / x = (x - 1) ^ 2 / x :=
-- by
--   congr
--   ring

-- -- Then $x + \frac{1}{x} - 2 = \frac{{(x-1)}^2}{x}$
-- def h2 (x : ℝ) (h : x > 0) : x + 1 / x - 2 = (x - 1) ^ 2 / x :=
-- by rw [h x h, h1 x]

-- -- Then $\frac{{(x-1)}^2}{x} \ge 0$
-- def h3 (x : ℝ) (h : x > 0) : (x - 1) ^ 2 / x ≥ 0 :=
-- by
--   have h_nonneg : (x - 1) ^ 2 ≥ 0 := by nlinarith
--   exact div_nonneg h_nonneg h.le

-- -- Then $x + \frac{1}{x} - 2 \ge 0$
-- def h4 (x : ℝ) (h : x > 0) : x + 1 / x - 2 ≥ 0 :=
-- by
--   rw [h2 x h]
--   exact h3 x h

-- -- Then $x + \frac{1}{x} \ge 2$
-- def h5 (x : ℝ) (h : x > 0) : x + 1 / x ≥ 2 :=
-- by
--   linarith [h4 x h]

-- end Example6


import Mathlib.Tactic.Linarith
import Mathlib.Tactic.Ring
import Mathlib.Tactic.FieldSimp
import Mathlib.Data.Real.Basic

namespace Example6

-- Let $x\in\mathbb{R}$
variable {x : ℝ}

-- Assume $x > 0$
variable (hx : x > 0)

-- Then $x + \frac{1}{x} - 2 = \frac{x^2 + 1 - 2x}{x}$
def step1 (x : ℝ) (hx : x > 0) : x + 1 / x - 2 = (x ^ 2 + 1 - 2 * x) / x :=
by
  field_simp [ne_of_gt hx]
  ring

-- Then $\frac{x^2 + 1 - 2x}{x} = \frac{{(x-1)}^2}{x}$
def step2 (x : ℝ) : (x ^ 2 + 1 - 2 * x) / x = (x - 1) ^ 2 / x :=
by
  congr
  ring

-- Then $x + \frac{1}{x} - 2 = \frac{{(x-1)}^2}{x}$
def step3 (x : ℝ) (hx : x > 0) : x + 1 / x - 2 = (x - 1) ^ 2 / x :=
by rw [step1 x hx, step2 x]

-- Then $\frac{{(x-1)}^2}{x} \ge 0$
def step4 (x : ℝ) (hx : x > 0) : (x - 1) ^ 2 / x ≥ 0 :=
by
  have h_nonneg : (x - 1) ^ 2 ≥ 0 := by nlinarith
  exact div_nonneg h_nonneg hx.le

-- Then $x + \frac{1}{x} - 2 \ge 0$
def step5 (x : ℝ) (hx : x > 0) : x + 1 / x - 2 ≥ 0 :=
by
  rw [step3 x hx]
  exact step4 x hx

-- Then $x + \frac{1}{x} \ge 2$
def step6 (x : ℝ) (hx : x > 0) : x + 1 / x ≥ 2 :=
by
  linarith [step5 x hx]

end Example6
