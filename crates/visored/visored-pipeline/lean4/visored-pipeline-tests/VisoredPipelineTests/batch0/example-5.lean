import Mathlib
syntax "attack" : tactic

macro_rules
| `(tactic| attack) => `(tactic|
  first
  | simp; done
  | ring; done
  | ring_nf; rw [Real.sq_sqrt]; ring; all_goals attack; done
  | nlinarith; done
  | apply sq_nonneg; all_goals attack; done
  | apply div_nonneg; all_goals attack; done
  | field_simp; ring; done
  | linarith; done
)

macro "obvious": tactic =>`(tactic|
  first
  | attack; done
  | congr; all_goals attack; done
  | gcongr; all_goals attack; done
  | fail "Could not prove this goal automatically. It might not be as obvious as you think!"
)
def h (a b : ℝ) : ((a + b) / (2 : ℝ)) ^ 2 ≤ (a ^ 2 + b ^ 2) / (2 : ℝ) := by
  have h1 : (a - b) ^ 2 ≥ (0 : ℝ) := by obvious
  have h2 : a ^ 2 - (2 : ℝ) * a * b + b ^ 2 ≥ (0 : ℝ) := by obvious
  have h3 : (2 : ℝ) * a * b ≤ a ^ 2 + b ^ 2 := by obvious
  have h4 : a ^ 2 + (2 : ℝ) * a * b + b ^ 2 ≤ (2 : ℝ) * (a ^ 2 + b ^ 2) := by calc
    a ^ 2 + (2 : ℝ) * a * b + b ^ 2 ≤ a ^ 2 + b ^ 2 + (2 : ℝ) * a * b := by obvious
    _ ≤ a ^ 2 + b ^ 2 + (a ^ 2 + b ^ 2) := by obvious
    _ = (2 : ℝ) * (a ^ 2 + b ^ 2) := by obvious
  have h5 : (a + b) ^ 2 ≤ (2 : ℝ) * (a ^ 2 + b ^ 2) := by obvious
  first
  | have h6 : (a + b) ^ 2 / (4 : ℝ) ≤ (a ^ 2 + b ^ 2) / (2 : ℝ) := by calc
    (a + b) ^ 2 / (4 : ℝ) = ((a + b) / (2 : ℝ)) ^ 2 := by obvious
    _ ≤ (2 : ℝ) * (a ^ 2 + b ^ 2) / (4 : ℝ) := by obvious
    _ = (a ^ 2 + b ^ 2) / (2 : ℝ) := by obvious
  | have h7 : ((a + b) / (2 : ℝ)) ^ 2 ≤ (a ^ 2 + b ^ 2) / (2 : ℝ) := by calc
    ((a + b) / (2 : ℝ)) ^ 2 = (a + b) ^ 2 / (4 : ℝ) := by obvious
    _ ≤ (2 : ℝ) * (a ^ 2 + b ^ 2) / (4 : ℝ) := by obvious
    _ = (a ^ 2 + b ^ 2) / (2 : ℝ) := by obvious
  obvious
