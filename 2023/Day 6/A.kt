import java.io.File

fun main() {
    val input = File("input.txt")
    // val input = File("example input.txt")

    val lines = input.readLines()
    val times = lines[0].split(':')[1].trim().split("\\s+".toRegex()).map { it.toInt() }
    val distances = lines[1].split(':')[1].trim().split("\\s+".toRegex()).map { it.toInt() }

    var valid_combination_product = 1

    for (i in 0 until times.size) {
        val time = times[i]
        val min_distance = distances[i]

        var valid_combinations = 0
        for (j in 0 until time) {
            if ((time - j)*j > min_distance) {
                ++valid_combinations
            }
        }
        valid_combination_product *= valid_combinations
    }
    println("Product of valid combinations: $valid_combination_product")
}