import Mathlib.Tactic.Ring
import Mathlib.Data.Real.Basic
import Mathlib.Data.Real.Sqrt

namespace Example

-- Let x ∈ ℝ, x ≥ 0.
variable {x : ℝ} (hx : x ≥ 0)

-- Then √x² + 1 - 2 √x = (√x - 1)².
def h1 : x + 1 - 2 * √x = (√x - 1) ^ 2 :=
by
  have hx' : x = (√x) ^ 2 := by
    exact Sqrt.sqrt_sq hx
  rw [hx']
  ring_nf

-- Then (√x - 1)² ≥ 0.
def h2 : (√x - 1) ^ 2 ≥ 0 :=
by
  apply sq_nonneg

-- Then x + 1 ≥ 2 √x.
def h3 : x + 1 ≥ 2 * √x :=
by
  have key : x + 1 - 2 * √x ≥ 0 := by
    rw [h1]
    exact h2
  exact sub_nonneg.mp key

end Example
