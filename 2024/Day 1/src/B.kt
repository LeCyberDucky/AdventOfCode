import java.io.File
import kotlin.math.abs

fun main() {
    // val input = File("../example input.txt")
    val input = File("../input.txt")

    val left = mutableListOf<Int>()
    val right = mutableListOf<Int>()

    input.forEachLine() { line ->
        // Splitting the two numbers on each line by whatever whitespace separates them
        val (leftValue, rightValue) = line.split("\\s+".toRegex()).map(String::toInt)
        left.add(leftValue)
        right.add(rightValue)
    }

    val bounds = (kotlin.math.min(left.min(), right.min())..kotlin.math.max(left.max(), right.max()))
    val occurences = MutableList(bounds.endInclusive - bounds.start + 1) {0}
    for (x in right) {
        val index = x - bounds.start
        ++occurences[index]
    }

    var sum = 0
    for (x in left) {
        val index = x - bounds.start
        sum += x * occurences[index]
    }

    println("Sum: $sum")
}