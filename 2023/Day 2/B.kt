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

    var game_power_sum = 0

    for (game in games) {
        val power = game["red"]!! * game["green"]!! * game["blue"]!!
        game_power_sum += power
    }

    println("Sum of game-powers: $game_power_sum")
}