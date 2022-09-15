import Specs.abstraction.Book
import Specs.syntax
import Specs.infer
import Specs.semantics
import Specs.vm

def book_specs : Book := { chapters:=[ch_syntax, ch_infer, ch_semantics, ch_vm] }
def hello := "world"