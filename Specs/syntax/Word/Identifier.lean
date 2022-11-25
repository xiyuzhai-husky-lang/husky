import Specs.syntax.Text

structure Identifier where
  data : String
  deriving DecidableEq

namespace Identifier
def huskyCode : Identifier -> String
  | _ => sorry
end Identifier