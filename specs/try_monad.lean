

/-!
# Monad Stacks
Before you read this: This chapter does not attempt to introduce the concept
of a monad in general, it assume this as a given and explains the monads
and constructs on top of them that are essential to Lean meta programming.
If you don't know what a monad is already you can read (TODO: Link to monad tutorial).
## Monad Transformers
Quite often in functional programming with monads one wants to have the power
of more than one monad available. Specifically in Lean meta programming it
is a very common pattern that we have some sort of read only input, an
execution context, available to us, as well as the ability to operate on
a mutable state. Both of these issues have monads that can solve them
separately already, namely the `Reader` monad for read only input and the
`StateM` monad for the mutable state. Now we can try to just "stack" them
ontop like this:`
-/
namespace Playground1
abbrev Context := Nat
abbrev State := String

abbrev MyM (α : Type) := Reader Context (StateM State α)

end Playground1

/-!
We can fix this by manually declaring the instance though:
-/
namespace Playground1

instance : Monad MyM where
  pure x := fun ctx state => (x, state)
  bind x f := fun ctx state =>
    let (res, newState) := x ctx state
    f res ctx newState

end Playground1

namespace Playground2
abbrev Context := Nat
abbrev State := String

-- `StateM` is defined as `StateM σ α = StateT σ Id α` so the following two are equivalent.
abbrev MyM  := ReaderT Context (StateT State Id)
abbrev MyM2  := ReaderT Context (StateM State)
-- This definition of `StateM` makes perfect sense since the `Id` monad has no effect,
-- so combining it with `StateT` will create a monad that has only the state effect.
-- Can you guess the definition of `Reader` based on `ReaderT`?

#synth Monad MyM -- monad instance!
#synth Monad MyM2 -- monad instance!

end Playground2



/-!
So that's the monad instance for free. The other thing that failed above,
was that we didn't have access to `read`, `get` and `set`, anymore, let's
see whether our monad transformers allow this:
-/
namespace Playground2

def testReader : MyM String := do
  let get ← read -- obtain our context
  pure s!"This is my context: {get}"

def testState (add : State) : MyM Unit := do
  let old ← get
  set s!"{old} + {add}"

def testStateChange : MyM String := do
  let _ ← testState "new stuff!"
  testState "more new stuff!"
  get

def MyM.run (context : Context) (state : State) (x : MyM α) : α :=
  Prod.fst <$> StateT.run (ReaderT.run x context) state

#eval MyM.run 12 "hello" testReader -- "This is my context: 12"
#eval MyM.run 12 "hello" testStateChange -- "hello + new stuff! + more new stuff!"

end Playground2