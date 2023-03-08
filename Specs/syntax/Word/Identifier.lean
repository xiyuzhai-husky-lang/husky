import Specs.syntax.Text

structure Ident where
  data : String
  deriving DecidableEq

namespace Ident
def huskyCode : Ident -> String
  | _ => sorry
end Ident