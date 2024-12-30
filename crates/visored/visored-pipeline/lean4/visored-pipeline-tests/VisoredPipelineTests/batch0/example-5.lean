import Mathlib
import Obvious
open Obvious

def h(a : ℝ)(b : ℝ) : ((a + b) / 2) ^ 2 ≤ (a ^ 2 + b ^ 2) / 2 := by
  have h1 : (a - b) ^ 2 ≥ 0 := by obvious
  have h2 : a ^ 2 - 2 * a * b + b ^ 2 ≥ 0 := by obvious
  have h3 : 2 * a * b ≤ a ^ 2 + b ^ 2 := by obvious
  have h4 : a ^ 2 + 2 * a * b + b ^ 2 ≤ 2 * (a ^ 2 + b ^ 2) := by calc
    a ^ 2 + 2 * a * b + b ^ 2 ≤ a ^ 2 + b ^ 2 + 2 * a * b := by obvious
    _ ≤ a ^ 2 + b ^ 2 + (a ^ 2 + b ^ 2) := by obvious
    _ = 2 * (a ^ 2 + b ^ 2) := by obvious
  have h5 : (a + b) ^ 2 ≤ 2 * (a ^ 2 + b ^ 2) := by obvious
  first
  | have h6 : (a + b) ^ 2 / 4 ≤ (a ^ 2 + b ^ 2) / 2 := by calc
    (a + b) ^ 2 / 4 = ((a + b) / 2) ^ 2 := by obvious
    _ ≤ 2 * (a ^ 2 + b ^ 2) / 4 := by obvious
    _ = (a ^ 2 + b ^ 2) / 2 := by obvious
  | have h7 : ((a + b) / 2) ^ 2 ≤ (a ^ 2 + b ^ 2) / 2 := by calc
    ((a + b) / 2) ^ 2 = (a + b) ^ 2 / 4 := by obvious
    _ ≤ 2 * (a ^ 2 + b ^ 2) / 4 := by obvious
    _ = (a ^ 2 + b ^ 2) / 2 := by obvious
  obvious
