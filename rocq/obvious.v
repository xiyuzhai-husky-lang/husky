Require Import Coq.setoid_ring.Ring_theory.
Require Import Coq.setoid_ring.Ring_tac.
Require Import Coq.Reals.R_sqrt.
Require Import Coq.micromega.Lra. 
Require Import Coq.Reals.Reals.
Require Import Coq.micromega.Lia. 

Ltac attack :=
  first [
    simpl; easy
  | ring; easy
  | ring_simplify; rewrite sqrt_square; ring; solve [attack]
  | nra (* Non-linear arithmetic *)
  | apply Rle_0_sqr; solve [attack]
  | apply Rle_mult_inv_pos; solve [attack]
  | field_simplify; ring; easy
  | lia (* Linear integer/arithmetic solver *)
  ].

(* Coq does not have gcongr. TODO: better congr *)
Ltac gcongr :=
  match goal with
  | [ |- ?f _ <= ?f _ ] => apply (Rle_trans _ _ _)
  | [ |- context[?x + ?y] ] => apply Rplus_le_compat
  end.


Ltac obvious :=
  first [
    attack; easy
  | congruence; solve [attack]
  | try gcongr; solve [attack] (* If you have a corresponding `gcongr` tactic or library *)
  | fail "Could not prove this goal automatically. It might not be as obvious as you think!"
  ].
