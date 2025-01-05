import Mathlib

namespace Example1
def h(x : ℝ) := by
  have h1 : x ^ 2 ≥ 0 := by apply sq_nonneg
  exact ()
end Example1

namespace Example2
def h(x : ℝ)(h1 : x ≥ 1) := by
  have h2 : x - 1 ≥ 0 := by obvious
  exact ()
end Example2

