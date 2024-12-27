import Mathlib
import Obvious
open Obvious

def h(x : ℝ)(h1 : x > 0) := by
  have h2 : x > 0 := by obvious
  have h3 : (x - 1) ^ 2 ≥ 0 := by obvious
  have h4 : x ^ 2 - 2 * x + 1 ≥ 0 := by obvious
  have h5 : (x ^ 2 - 2 * x + 1) / x ≥ 0 := by calc
      (x ^ 2 - 2 * x + 1) / x = x ^ 2 / x - 2 * x / x + 1 / x := by obvious
    _ = x - 2 + 1 / x := by obvious
    _ ≥ 0 := by obvious
  have h6 : x + 1 / x ≥ 2 := by obvious
  obvious
