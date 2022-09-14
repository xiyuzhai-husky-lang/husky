inductive ConfigKeyword
  | Task
  deriving DecidableEq

namespace ConfigKeyword
def kindName : ConfigKeyword -> String
  | Task => "Task"

def huskyCode : ConfigKeyword -> String
  | Task => "task"

instance : ToString ConfigKeyword where
  toString : ConfigKeyword -> String
  | kw => s!"ConfigKeyword::{kw.kindName}"

def rustTests : String :=
  s!"
fn _test_enumeration(a: Type) \{
    match a \{
        case _ => ()
    }
}
  "
end ConfigKeyword