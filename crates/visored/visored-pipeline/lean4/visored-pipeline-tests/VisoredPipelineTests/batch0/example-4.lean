import Mathlib
import Obvious
open Obvious

def h(x : ℝ)(h1 : x > (0 : ℝ)) : x + (1 : ℝ) / x ≥ (2 : ℝ) := by
  have h2 : x > (0 : ℝ) := by obvious
  first
  | have h3 : (x - (1 : ℝ)) ^ 2 ≥ (0 : ℝ) := by calc
    (x - (1 : ℝ)) ^ 2 = x ^ 2 - (2 : ℝ) * x + (1 : ℝ) := by obvious
    _ ≥ (0 : ℝ) := by obvious
  | have h4 : x ^ 2 - (2 : ℝ) * x + (1 : ℝ) ≥ (0 : ℝ) := by calc
    x ^ 2 - (2 : ℝ) * x + (1 : ℝ) = (x - (1 : ℝ)) ^ 2 := by obvious
    _ ≥ (0 : ℝ) := by obvious
  have h5 : x ^ 2 + (1 : ℝ) ≥ (2 : ℝ) * x := by obvious
  have h6 : x + (1 : ℝ) / x ≥ (2 : ℝ) := by obvious
  obvious
