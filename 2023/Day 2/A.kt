import java.io.File
import kotlin.math.max

fun main() {
    val input = File("input.txt")
    // val input = File("example input.txt")

    val games: MutableList<MutableMap<String, Int>> = mutableListOf()

    input.forEachLine() { line ->
        val cube_counts = mutableMapOf("red" to 0, "green" to 0, "blue" to 0)
        val hands = line.split(':')[1].split(';')
        for (hand in hands) {
            val samples = hand.split(',')
            for (sample in samples) {
                val sample_components = sample.trim().split(' ')
                val count = sample_components[0].toInt()
                val color = sample_components[1]
                cube_counts[color] = max(cube_counts[color]!!, count)
            }
        }
        games.add(cube_counts)
    }

    var valid_game_sum = 0
    val bag = mapOf("red" to 12, "green" to 13, "blue" to 14)
    for ((id, game) in games.withIndex()) {
        var valid = true
        for ((color, limit) in bag) {
            if (game[color]!! > limit) {
                valid = false
            }
        }
        if (valid) {
            valid_game_sum += (id + 1)
        }
    }

    println("ID-sum of valid games: $valid_game_sum")
}