import spinal.core._
import spinal.lib._

class AND_Gate extends Component {

  /**
    * This is the component definition that corresponds to
    * the VHDL entity of the component
    */
  val io = new Bundle {
    val a = in Bool()
    val b = in Bool()
    val c = out Bool()
  }

  // Here we define some asynchronous logic
  io.c := io.a & io.b
}

object AND_Gate {
  // Let's go
  def main(args: Array[String]) {
    SpinalVhdl(new AND_Gate)
  }
}