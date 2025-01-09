import Mathlib
import Obvious
open Obvious

def h(a : ℝ)(b : ℝ)(c : ℝ) : a ^ 2 + b ^ 2 + c ^ 2 ≥ a * b + b * c + c * a := by
  first
  | have h1 : (a - b) ^ 2 + (b - c) ^ 2 + (c - a) ^ 2 = (2 : ℝ) * (a ^ 2 + b ^ 2 + c ^ 2 - a * b - b * c - c * a) := by calc
    (a - b) ^ 2 + (b - c) ^ 2 + (c - a) ^ 2 = a ^ 2 - (2 : ℝ) * a * b + b ^ 2 + b ^ 2 - (2 : ℝ) * b * c + c ^ 2 + c ^ 2 - (2 : ℝ) * c * a + a ^ 2 := by obvious
    _ = (2 : ℝ) * (a ^ 2 + b ^ 2 + c ^ 2 - a * b - b * c - c * a) := by obvious
  | have h2 : (2 : ℝ) * (a ^ 2 + b ^ 2 + c ^ 2 - a * b - b * c - c * a) = (a - b) ^ 2 + (b - c) ^ 2 + (c - a) ^ 2 := by calc
    (2 : ℝ) * (a ^ 2 + b ^ 2 + c ^ 2 - a * b - b * c - c * a) = a ^ 2 - (2 : ℝ) * a * b + b ^ 2 + b ^ 2 - (2 : ℝ) * b * c + c ^ 2 + c ^ 2 - (2 : ℝ) * c * a + a ^ 2 := by obvious
    _ = (a - b) ^ 2 + (b - c) ^ 2 + (c - a) ^ 2 := by obvious
  have h3 : (a - b) ^ 2 + (b - c) ^ 2 + (c - a) ^ 2 ≥ (0 : ℝ) := by obvious
  have h4 : (2 : ℝ) * (a ^ 2 + b ^ 2 + c ^ 2 - a * b - b * c - c * a) ≥ (0 : ℝ) := by obvious
  have h5 : a ^ 2 + b ^ 2 + c ^ 2 ≥ a * b + b * c + c * a := by obvious
  obvious
