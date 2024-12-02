import Mathlib.Tactic.Linarith
import Mathlib.Tactic.Ring
import Mathlib.Tactic.FieldSimp
import Mathlib.Data.Real.Basic

namespace Example5

variable {x : ℝ} (h : x > 0)

def h1 (x : ℝ) (h : x > 0) : x + 1 / x - 2 = (x ^ 2 + 1 - 2 * x) / x :=
by
  field_simp [ne_of_gt h]
  ring

def h2 (x : ℝ) : (x ^ 2 + 1 - 2 * x) / x = (x - 1) ^ 2 / x :=
by
  congr
  ring

def h3 (x : ℝ) (h : x > 0) : x + 1 / x - 2 = (x - 1) ^ 2 / x :=
by rw [h1 x h, h2 x]

def h4 (x : ℝ) (h : x > 0) : (x - 1) ^ 2 / x ≥ 0 :=
by
  have h_nonneg : (x - 1) ^ 2 ≥ 0 := by nlinarith
  exact div_nonneg h_nonneg h.le

def h5 (x : ℝ) (h : x > 0) : x + 1 / x - 2 ≥ 0 :=
by
  rw [h3 x h]
  exact h4 x h

def h6 (x : ℝ) (h : x > 0) : x + 1 / x ≥ 2 :=
by
  linarith [h5 x h]

end Example5
