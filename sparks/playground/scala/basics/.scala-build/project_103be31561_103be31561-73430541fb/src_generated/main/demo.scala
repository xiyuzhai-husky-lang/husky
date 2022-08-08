



object demo {
/*<script>*/def niceArgs
/*</script>*/ /*<generated>*/
def args = demo_sc.args$
  /*</generated>*/
}

object demo_sc {
  private var args$opt0 = Option.empty[Array[String]]
  def args$set(args: Array[String]): Unit = {
    args$opt0 = Some(args)
  }
  def args$opt: Option[Array[String]] = args$opt0
  def args$: Array[String] = args$opt.getOrElse {
    sys.error("No arguments passed to this script")
  }
  def main(args: Array[String]): Unit = {
    args$set(args)
    demo.hashCode() // hasCode to clear scalac warning about pure expression in statement position
  }
}

