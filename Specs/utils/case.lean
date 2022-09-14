import Specs.abstraction.Enumerable
inductive Case  -- Uppercase strings are delimited by spaces and all characters are uppercase.
    -- * Boundaries: [Space](`Boundary::Space`)
    -- * Pattern: [Uppercase](`Pattern::Uppercase`)
    -- * Delimeter: Space
    --
    -- ```
    -- use convert_case::{Case, Casing};
    -- assert_eq!("MY VARIABLE NAME", "My variable NAME".to_case(Case::Upper))
    -- ```
    | Upper

    -- Lowercase strings are delimited by spaces and all characters are lowercase.
    -- * Boundaries: [Space](`Boundary::Space`)
    -- * Pattern: [Lowercase](`Pattern::Lowercase`)
    -- * Delimeter: Space
    --
    -- ```
    -- use convert_case::{Case, Casing};
    -- assert_eq!("my variable name", "My variable NAME".to_case(Case::Lower))
    -- ```
    
    | Lower

    -- Title case strings are delimited by spaces. Only the leading character of
    -- each word is uppercase.  No inferences are made about language, so words
    -- like "as", "to", and "for" will still be capitalized.
    -- * Boundaries: [Space](`Boundary::Space`)
    -- * Pattern: [Capital](`Pattern::Capital`)
    -- * Delimeter: Space
    --
    -- ```
    -- use convert_case::{Case, Casing};
    -- assert_eq!("My Variable Name", "My variable NAME".to_case(Case::Title))
    -- ```
    | Title

    -- Toggle case strings are delimited by spaces.  All characters are uppercase except
    -- for the leading character of each word, which is lowercase.
    -- * Boundaries: [Space](`Boundary::Space`)
    -- * Pattern: [Toggle](`Pattern::Toggle`)
    -- * Delimeter: Space
    --
    -- ```
    -- use convert_case::{Case, Casing};
    -- assert_eq!("mY vARIABLE nAME", "My variable NAME".to_case(Case::Toggle))
    -- ```
    | Toggle

    -- Camel case strings are lowercase, but for every word _except the first_ the
    -- first letter is capitalized.
    -- * Boundaries: [LowerUpper](Boundary::LowerUpper), [DigitUpper](Boundary::DigitUpper),
    -- [UpperDigit](Boundary::UpperDigit), [DigitLower](Boundary::DigitLower),
    -- [LowerDigit](Boundary::LowerDigit), [Acronym](Boundary::Acronym)
    -- * Pattern: [Camel](`Pattern::Camel`)
    -- * Delimeter: No delimeter
    --
    -- ```
    -- use convert_case::{Case, Casing};
    -- assert_eq!("myVariableName", "My variable NAME".to_case(Case::Camel))
    -- ```
   | Camel

    -- Pascal case strings are lowercase, but for every word the
    -- first letter is capitalized.
    -- * Boundaries: [LowerUpper](Boundary::LowerUpper), [DigitUpper](Boundary::DigitUpper),
    -- [UpperDigit](Boundary::UpperDigit), [DigitLower](Boundary::DigitLower),
    -- [LowerDigit](Boundary::LowerDigit), [Acronym](Boundary::Acronym)
    -- * Pattern: [Capital](`Pattern::Capital`)
    -- * Delimeter: No delimeter
    --
    -- ```
    -- use convert_case::{Case, Casing};
    -- assert_eq!("MyVariableName", "My variable NAME".to_case(Case::Pascal))
    -- ```
    | Pascal

    -- Upper camel case is an alternative name for [Pascal case](Case::Pascal).
   | UpperCamel

    -- Snake case strings are delimited by underscores `_` and are all lowercase.
    -- * Boundaries: [Underscore](Boundary::Underscore)
    -- * Pattern: [Lowercase](Pattern::Lowercase)
    -- * Delimeter: Underscore `_`
    --
    -- ```
    -- use convert_case::{Case, Casing};
    -- assert_eq!("my_variable_name", "My variable NAME".to_case(Case::Snake))
    -- ```
   | Snake

    -- Upper snake case strings are delimited by underscores `_` and are all uppercase.
    -- * Boundaries: [Underscore](Boundary::Underscore)
    -- * Pattern: [Uppercase](Pattern::Uppercase)
    -- * Delimeter: Underscore `_`
    --
    -- ```
    -- use convert_case::{Case, Casing};
    -- assert_eq!("MY_VARIABLE_NAME", "My variable NAME".to_case(Case::UpperSnake))
    -- ```
  | UpperSnake

    -- Screaming snake case is an alternative name for [upper snake case](Case::UpperSnake).
  | ScreamingSnake

    -- Kebab case strings are delimited by hyphens `-` and are all lowercase.
    -- * Boundaries: [Hyphen](Boundary::Hyphen)
    -- * Pattern: [Lowercase](Pattern::Lowercase)
    -- * Delimeter: Hyphen `-`
    --
    -- ```
    -- use convert_case::{Case, Casing};
    -- assert_eq!("my-variable-name", "My variable NAME".to_case(Case::Kebab))
    -- ```
  | Kebab

    -- Cobol case strings are delimited by hyphens `-` and are all uppercase.
    -- * Boundaries: [Hyphen](Boundary::Hyphen)
    -- * Pattern: [Uppercase](Pattern::Uppercase)
    -- * Delimeter: Hyphen `-`
    --
    -- ```
    -- use convert_case::{Case, Casing};
    -- assert_eq!("MY-VARIABLE-NAME", "My variable NAME".to_case(Case::Cobol))
    -- ```
  | Cobol

    -- Upper kebab case is an alternative name for [Cobol case](Case::Cobol).
  | UpperKebab

    -- Train case strings are delimited by hyphens `-`.  All characters are lowercase
    -- except for the leading character of each word.
    -- * Boundaries: [Hyphen](Boundary::Hyphen)
    -- * Pattern: [Capital](Pattern::Capital)
    -- * Delimeter: Hyphen `-`
    --
    -- ```
    -- use convert_case::{Case, Casing};
    -- assert_eq!("My-Variable-Name", "My variable NAME".to_case(Case::Train))
    -- ```
  | Train

    -- Flat case strings are all lowercase, with no delimiter. Note that word boundaries are lost.
    -- * Boundaries: No boundaries
    -- * Pattern: [Lowercase](Pattern::Lowercase)
    -- * Delimeter: No delimeter
    --
    -- ```
    -- use convert_case::{Case, Casing};
    -- assert_eq!("myvariablename", "My variable NAME".to_case(Case::Flat))
    -- ```
  | Flat

    -- Upper flat case strings are all uppercase, with no delimiter. Note that word boundaries are lost.
    -- * Boundaries: No boundaries
    -- * Pattern: [Uppercase](Pattern::Uppercase)
    -- * Delimeter: No delimeter
    --
    -- ```
    -- use convert_case::{Case, Casing};
    -- assert_eq!("MYVARIABLENAME", "My variable NAME".to_case(Case::UpperFlat))
    -- ```
  | UpperFlat

    -- Alternating case strings are delimited by spaces.  Characters alternate between uppercase
    -- and lowercase.
    -- * Boundaries: [Space](Boundary::Space)
    -- * Pattern: [Alternating](Pattern::Alternating)
    -- * Delimeter: Space
    --
    -- ```
    -- use convert_case::{Case, Casing};
    -- assert_eq!("mY vArIaBlE nAmE", "My variable NAME".to_case(Case::Alternating));
    -- ```
  | Alternating

    -- Random case strings are delimited by spaces and characters are
    -- randomly upper case or lower case.  This uses the `rand` crate
    -- and is only available with the "random" feature.
    -- * Boundaries: [Space](Boundary::Space)
    -- * Pattern: [Random](Pattern::Random)
    -- * Delimeter: Space
    --
    -- ```
    -- use convert_case::{Case, Casing};
    -- let new = "My variable NAME".to_case(Case::Random);
    -- ```
    -- String `new` could be "My vaRIAbLE nAme" for example.
  | Random

    -- Pseudo-random case strings are delimited by spaces and characters are randomly
    -- upper case or lower case, but there will never more than two consecutive lower
    -- case or upper case letters in a row.  This uses the `rand` crate and is
    -- only available with the "random" feature.
    -- * Boundaries: [Space](Boundary::Space)
    -- * Pattern: [PseudoRandom](Pattern::PseudoRandom)
    -- * Delimeter: Space
    --
    -- ```
    -- use convert_case::{Case, Casing};
    -- let new = "My variable NAME".to_case(Case::Random);
    -- ```
    -- String `new` could be "mY vArIAblE NamE" for example.
  | PseudoRandom
  deriving DecidableEq

namespace Case
instance : Enumerable Case where
  enumeration := [
    Upper,
    Lower,
    Title,
    Toggle,
    Camel,
    Pascal,
    UpperCamel,
    Snake,
    UpperSnake,
    ScreamingSnake,
    Kebab,
    Cobol,
    UpperKebab,
    Train,
    Flat,
    UpperFlat,
    Alternating,
    Random,
    PseudoRandom
  ]
  hvalid := by
    apply And.intro
    apply rfl
    intro a
    cases a with
    | _ => rfl
end Case

inductive Boundary

inductive Pattern

structure Converter where
  boundaries: List Boundary
  pattern: Option Pattern
  delim: String

structure StateConverter (α : Type) where
    s: α
    conv: Converter

structure ToCase (α : Type) where
    toCase : α -> Case -> String
    fromCase : α -> Case -> StateConverter α
    withBoundaries: α -> List Boundary -> StateConverter α
    isCase : α -> Case -> Bool