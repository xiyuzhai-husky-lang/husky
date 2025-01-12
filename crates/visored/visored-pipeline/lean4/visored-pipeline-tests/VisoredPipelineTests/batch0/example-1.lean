import Mathlib
import Obvious
open Obvious

def h (a b : ℝ) : (a + b) ^ 2 ≥ (0 : ℝ) := by
  have h1 : (a + b) ^ 2 ≥ (0 : ℝ) := by obvious
  obvious
