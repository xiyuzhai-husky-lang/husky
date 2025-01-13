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

macro "in_set" : term => `(true)

def h (a b : ℝ) (h1 : a > (0 : ℝ)) (h2 : b > (0 : ℝ)) : a / b + b / a ≥ (2 : ℝ) := by
  have h3 : a > (0 : ℝ) := by old_main_hypothesis
  have h4 : b > (0 : ℝ) := by old_main_hypothesis
  first
  | have h5 : (√ (a / b) - √ (b / a)) ^ 2 ≥ (0 : ℝ) := by calc
    (√ (a / b) - √ (b / a)) ^ 2 = √ (a / b) ^ 2 - (2 : ℝ) * √ (a / b) * √ (b / a) + √ (b / a) ^ 2 := by obvious
    _ = a / b - (2 : ℝ) + b / a := by obvious
    _ ≥ (0 : ℝ) := by obvious
  | have h6 : a / b - (2 : ℝ) + b / a ≥ (0 : ℝ) := by calc
    a / b - (2 : ℝ) + b / a = √ (a / b) ^ 2 - (2 : ℝ) * √ (a / b) * √ (b / a) + √ (b / a) ^ 2 := by obvious
    _ = (√ (a / b) - √ (b / a)) ^ 2 := by obvious
    _ ≥ (0 : ℝ) := by obvious
  have h7 : a / b - (2 : ℝ) + b / a ≥ (0 : ℝ) := by obvious
  have h8 : a / b + b / a ≥ (2 : ℝ) := by term_equivalent
  obvious
