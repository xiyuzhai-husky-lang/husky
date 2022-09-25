inductive Animal
  | Dog (height : Nat)
  | Cat

def Animal.getName : Animal -> String
  | Dog _ => "Dog"
  | Cat => "Cat"

def Animal.isEqual : Animal -> Animal -> Bool
  -- Dog
  | Dog _, Dog _ => True
  | Dog _, Cat => False

  -- Cat
  | Cat, Cat => True
  | Cat, Dog _ => False

#eval Animal.getName Animal.Cat

def isPrefixOf (a b : List α) : Prop :=
  ∃ c : List α, a ++ c = b

def isPrefixOfDec [DecidableEq α]: List α -> List α -> Bool
  | [], _ => true
  | _::_, [] => false
  | a::as, b::bs => a == b && isPrefixOfDec as bs

theorem is_prefix_of_dec_is_correct [DecidableEq α]
  : ∀ a b : List α, isPrefixOfDec a b -> isPrefixOf a b := by
  intro a b
  intro h
  match a, b with
  | [], x =>
    simp[isPrefixOfDec]
    simp[isPrefixOf]
    exact Exists.intro x (by simp)
  | _::_, [] =>
    simp[isPrefixOfDec]
    contradiction
  | a::as, b::bs =>
    simp[isPrefixOfDec] at h
    let hl := And.left h
    let hr := And.right h
    let hd := is_prefix_of_dec_is_correct as bs hr
    simp[isPrefixOf]
    simp[isPrefixOf] at hd
    let ⟨x, hdx⟩ := hd
    let hldx := And.intro hl hdx
    exact Exists.intro x hldx

#eval isPrefixOfDec "Animal".toList "Animals".toList
  -- sorry

theorem is_prefix_of_is_transitive
  : ∀ a b c : String, a.isPrefixOf b && b.isPrefixOf c -> a.isPrefixOf c := by
  intro a
  match a with
  | "" => sorry
  | _ => sorry