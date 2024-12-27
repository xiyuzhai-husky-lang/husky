import Mathlib
import Obvious
open Obvious

def h(x : ℝ)(y : ℝ)(h1 : x > 0)(h2 : y > 0) := by
  have h3 : (√ x - √ y) ^ 2 ≥ 0 := by obvious
  have h4 : x - 2 * √ x * y + y ≥ 0 := by calc
      x - 2 * √ x * y + y = (√ x - √ y) ^ 2 := by obvious
    _ ≥ 0 := by obvious
  have h5 : x + y ≥ 2 * √ x * y := by obvious
  have h6 : (x + y) / 2 ≥ √ x * y := by obvious
  obvious
