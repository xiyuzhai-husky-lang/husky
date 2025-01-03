import Mathlib
import Obvious
open Obvious

namespace Example1
def h(x : ℝ)(y : ℝ) := by
  have h1 : 4 * (x ^ 2) + y ^ 2 / 4 - 2 * x * y = (2 * x) ^ 2 + (y / 2) ^ 2 - 2 * (2 * x) * (y / 2) := by obvious
  have h2 : (2 * x) ^ 2 + (y / 2) ^ 2 - 2 * (2 * x) * (y / 2) = (2 * x - y / 2) ^ 2 := by obvious
  have h3 : (2 * x - y / 2) ^ 2 ≥ 0 := by obvious
  have h4 : 4 * (x ^ 2) + y ^ 2 / 4 - 2 * x * y ≥ 0 := by obvious
  have h5 : 4 * (x ^ 2) + y ^ 2 / 4 ≥ 2 * x * y := by obvious
  exact ()
end Example1
