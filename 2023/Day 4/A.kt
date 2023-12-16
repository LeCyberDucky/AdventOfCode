import java.io.File
import kotlin.math.max
import kotlin.math.pow

fun main() {
    val input = File("input.txt")
    // val input = File("example input.txt")

    var points = 0.0
    input.forEachLine() { card ->
        val card_parts = card.split('|')
        // Regex to split on one or more spaces because the input can contain sequences of multiple spaces
        val winning_numbers = card_parts[0].split(':')[1].trim().split("\\s+".toRegex()).map { it.toInt() }
        val own_numbers = card_parts[1].trim().split("\\s+".toRegex()).map { it.toInt() }

        var matches = 0
        for (number in own_numbers) {
            if (number in winning_numbers) {
                ++matches
            }
        }

        if (matches > 0) {
            points += 2.0.pow(matches - 1)
        }
    }
    println("Points: ${points.toInt()}")
}