import Mathlib
import Obvious
open Obvious

def h(x : ℝ)(y : ℝ)(h1 : x ≥ 0)(h2 : y ≥ 0) : (x + y) / 2 ≥ √ (x * y) := by
  first
  | have h3 : (√ x - √ y) ^ 2 = x - 2 * √ (x * y) + y := by calc
    (√ x - √ y) ^ 2 = √ x ^ 2 - 2 * √ x * √ y + √ y ^ 2 := by obvious
    _ = x - 2 * √ (x * y) + y := by obvious
  | have h4 : x - 2 * √ (x * y) + y = (√ x - √ y) ^ 2 := by calc
    x - 2 * √ (x * y) + y = √ x ^ 2 - 2 * √ x * √ y + √ y ^ 2 := by obvious
    _ = (√ x - √ y) ^ 2 := by obvious
  have h5 : (√ x - √ y) ^ 2 ≥ 0 := by obvious
  have h6 : x - 2 * √ (x * y) + y ≥ 0 := by obvious
  have h7 : x + y ≥ 2 * √ (x * y) := by obvious
  have h8 : (x + y) / 2 ≥ √ (x * y) := by obvious
  obvious
