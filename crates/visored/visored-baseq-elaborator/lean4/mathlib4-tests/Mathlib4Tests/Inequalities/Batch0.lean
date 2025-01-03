import Mathlib

namespace Example1
def h(x : ℝ) := by
  have h1 : x ^ 2 ≥ 0 := by apply sq_nonneg
  exact ()
end Example1

