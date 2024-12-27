import Mathlib
import Obvious
open Obvious

def h(a : ℝ)(b : ℝ) := by
  have h1 : (a ^ 2 + b ^ 2) / 2 - ((a + b) / 2) ^ 2 ≥ 0 := by calc
      (a ^ 2 + b ^ 2) / 2 - ((a + b) / 2) ^ 2 = (2 * (a ^ 2) + 2 * (b ^ 2)) / 4 - (a ^ 2 + 2 * a * b + b ^ 2) / 4 := by obvious
    _ = (2 * (a ^ 2) + 2 * (b ^ 2) - (a ^ 2 + 2 * a * b + b ^ 2)) / 4 := by obvious
    _ = (a ^ 2 - 2 * a * b + b ^ 2) / 4 := by obvious
    _ = (a - b) ^ 2 / 4 := by obvious
    _ ≥ 0 := by obvious
  have h2 : ((a + b) / 2) ^ 2 ≤ (a ^ 2 + b ^ 2) / 2 := by obvious
  exact ()
