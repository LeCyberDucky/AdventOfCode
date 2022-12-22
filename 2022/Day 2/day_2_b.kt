import java.io.File
import kotlin.math.max

fun main() {
    val input = File("input.txt")
    var score = 0

    input.forEachLine() {
        if (it.length == 3) {
            var (opponent, result) = it.split(" ")

            when (opponent) {
                "A" -> { // Rock
                    if (result == "X") { // Lose -> scissors
                        score += (0 + 3)
                    } else if (result == "Y") { // Draw -> rock
                        score += (3 + 1)
                    } else if (result == "Z") { // Win -> paper
                        score += (6 + 2)
                    }
                }
                "B" -> { // Paper
                    if (result == "X") { // Lose -> rock
                        score += (0 + 1)
                    } else if (result == "Y") { // Draw -> paper
                        score += (3 + 2)
                    } else if (result == "Z") { // Win -> scissors
                        score += (6 + 3)
                    }
                } 
                "C" -> { // Scissors
                    if (result == "X") { // Lose -> paper
                        score += (0 + 2)
                    } else if (result == "Y") { // Draw -> scissors
                        score += (3 + 3)
                    } else if (result == "Z") { // Win -> rock
                        score += (6 + 1)
                    }
                }
            }
        }
    }

     println("Total score: $score")
}