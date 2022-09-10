inductive ConfigKeyword
  | Task
  deriving BEq

namespace ConfigKeyword
def toRustVersion : ConfigKeyword -> String
  | Task => "ConfigKeyword::Task"

instance : ToString ConfigKeyword where
  toString : ConfigKeyword -> String
  | Task => "task"

def rustTests : String :=
  s!"
fn _test_enumeration(a: Type) \{
    match a \{
        case _ => ()
    }
}
  "
end ConfigKeyword