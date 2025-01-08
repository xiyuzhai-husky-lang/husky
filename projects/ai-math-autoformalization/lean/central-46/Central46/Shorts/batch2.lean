import Mathlib
import Obvious
open Obvious

namespace Example1
def h(x : ℝ)(y : ℝ) := by
  have h1 : (4 : ℝ) * (x ^ 2) + y ^ 2 / (4 : ℝ) - (2 : ℝ) * x * y = ((2 : ℝ) * x) ^ 2 + (y / (2 : ℝ)) ^ 2 - (2 : ℝ) * ((2 : ℝ) * x) * (y / (2 : ℝ)) := by obvious
  have h2 : ((2 : ℝ) * x) ^ 2 + (y / (2 : ℝ)) ^ 2 - (2 : ℝ) * ((2 : ℝ) * x) * (y / (2 : ℝ)) = ((2 : ℝ) * x - y / (2 : ℝ)) ^ 2 := by obvious
  have h3 : ((2 : ℝ) * x - y / (2 : ℝ)) ^ 2 ≥ (0 : ℝ) := by obvious
  have h4 : (4 : ℝ) * (x ^ 2) + y ^ 2 / (4 : ℝ) - (2 : ℝ) * x * y ≥ (0 : ℝ) := by obvious
  have h5 : (4 : ℝ) * (x ^ 2) + y ^ 2 / (4 : ℝ) ≥ (2 : ℝ) * x * y := by obvious
  exact ()
end Example1
