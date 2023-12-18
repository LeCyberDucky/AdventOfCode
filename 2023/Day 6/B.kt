import java.io.File

fun main() {
    val input = File("input.txt")
    // val input = File("example input.txt")

    val lines = input.readLines()
    val time = lines[0].split(':')[1].trim().filter { !it.isWhitespace() }.toLong()
    val min_distance = lines[1].split(':')[1].trim().filter { !it.isWhitespace() }.toLong()

    var valid_combinations = 0
    for (i in 0 until time) {
        if ((time - i)*i > min_distance) {
            ++valid_combinations
        }
    }

    println("Valid combinations: $valid_combinations")
}