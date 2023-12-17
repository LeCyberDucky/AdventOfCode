import java.io.File

fun main() {
    val input = File("input.txt")
    // val input = File("example input.txt")

    val lines = input.readLines()
    var card_counts = MutableList(lines.size) {1}

    for ((i, card) in lines.withIndex()) {
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

        for (j in 1..(matches)) {
            card_counts[i + j] += card_counts[i]
        }
    }

    println("Card counts: $card_counts")
    println("Total number of cards: ${card_counts.sum()}")
}