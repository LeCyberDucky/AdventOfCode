import java.io.File
import kotlin.math.max

fun main() {
    val input = File("input.txt")
    var score = 0

    input.forEachLine() {
        if (it.length == 3) {
            var (opponent, own) = it.split(" ")
            when (own) {
                "X" -> { // Rock
                    score += 1
                    if (opponent == "A") { // Rock
                        score += 3
                    } else if (opponent == "B") { // Paper
                        score += 0
                    } else if (opponent == "C") { // Scissors
                        score += 6
                    }
                }
                "Y" -> { // Paper
                    score += 2
                    if (opponent == "A") { // Rock
                        score += 6
                    } else if (opponent == "B") { // Paper
                        score += 3
                    } else if (opponent == "C") { // Scissors
                        score += 0
                    }
                }
                "Z" -> { // Scissors
                    score += 3
                    if (opponent == "A") { // Rock
                        score += 0
                    } else if (opponent == "B") { // Paper
                        score += 6
                    } else if (opponent == "C") { // Scissors
                        score += 3
                    }
                }
            }
        }
    }

     println("Total score: $score")
}