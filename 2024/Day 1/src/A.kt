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

    left.sort()
    right.sort()

    val delta = left.zip(right) {left, right -> abs(left - right)}

    println("Left: $left")
    println("Right: $right")
    println("Delta: $delta")
    println("Sum of deltas: ${delta.sum()}")
}