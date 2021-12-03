import scala.math.Ordering.Implicits.infixOrderingOps

def getLines(filename: String): Stream[String] =
  io.Source.fromResource(filename).getLines().toStream

lazy val depths = getLines("day1.txt")
  .map(_.toInt)

def countIncreases[T : Ordering](xs: Iterator[T]): Int =
  xs.sliding(2).count { case Seq(a, b) => a > b; case _ => false }

val day1_part1_solution = countIncreases(depths.toIterator)

val day1_part2_solution = countIncreases(
  depths
  .sliding(3)
  .collect { case Seq(a, b, c) => a + b + c }
)
