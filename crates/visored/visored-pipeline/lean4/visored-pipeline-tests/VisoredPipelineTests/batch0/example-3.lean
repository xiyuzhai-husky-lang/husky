import Mathlib
import Obvious
open Obvious

def h(a : ℝ)(b : ℝ)(c : ℝ) := by
  first
  | have h1 : (a - b) ^ 2 + (b - c) ^ 2 + (c - a) ^ 2 ≥ 0 := by calc
      (a - b) ^ 2 + (b - c) ^ 2 + (c - a) ^ 2 = a ^ 2 - 2 * a * b + b ^ 2 + (b ^ 2 - 2 * b * c + c ^ 2) + (c ^ 2 - 2 * c * a + a ^ 2) := by obvious
    _ = 2 * (a ^ 2) + 2 * (b ^ 2) + 2 * (c ^ 2) - 2 * a * b - 2 * b * c - 2 * c * a := by obvious
    _ ≥ 0 := by obvious
  | have h1 : 2 * (a ^ 2) + 2 * (b ^ 2) + 2 * (c ^ 2) - 2 * a * b - 2 * b * c - 2 * c * a ≥ 0 := by calc
    2 * (a ^ 2) + 2 * (b ^ 2) + 2 * (c ^ 2) - 2 * a * b - 2 * b * c - 2 * c * a = a ^ 2 - 2 * a * b + b ^ 2 + (b ^ 2 - 2 * b * c + c ^ 2) + (c ^ 2 - 2 * c * a + a ^ 2) := by obvious
    _ = (a - b) ^ 2 + (b - c) ^ 2 + (c - a) ^ 2 := by obvious
    _ ≥ 0 := by obvious
  have h2 : a ^ 2 + b ^ 2 + c ^ 2 - a * b - b * c - c * a ≥ 0 := by obvious
  show a ^ 2 + b ^ 2 + c ^ 2 ≥ a * b + b * c + c * a by obvious
  have h3 : a ^ 2 + b ^ 2 + c ^ 2 ≥ a * b + b * c + c * a := by obvious
  exact ()
