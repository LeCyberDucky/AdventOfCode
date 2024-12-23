import java.io.File

fun main() {
    // val input = File("example input.txt").readText()
    val input = File("input.txt").readText()

    val regex = Regex("""mul\((\d+),(\d+)\)""")

    var sum = 0
    val matches = regex.findAll(input)
    for (match in matches) {
        val (_, a, b) = match.groupValues
        sum += a.toInt()*b.toInt()
    }

    println("Sum of results: $sum")
}