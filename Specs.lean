import Specs.abstraction.Book
import Specs.syntax
import Specs.infer
import Specs.semantics
import Specs.comptime
import Specs.runtime
import Specs.devtime
import Specs.vm
import Specs.kernel

def book_specs : Book := { title := "Husky Language Specification", chapters:=[ch_syntax, ch_infer, ch_semantics, ch_vm] }
def hello := "world"