import java.io.File

fun main() {
    // val input = File("example input.txt").readText()
    val input = File("input.txt").readText()

    val regex = Regex("""mul\((\d+),(\d+)\)|do\(\)|don't\(\)""")

    var sum = 0
    val matches = regex.findAll(input)
    var mul_enabled = true
    for (match in matches) {
        val values = match.groupValues
        val instruction = values[0]
        if (instruction == "do()") {
            mul_enabled = true
        } else if (instruction == "don't()") {
            mul_enabled = false
        } else if (mul_enabled) {
            val (_, a, b) = values
            sum += a.toInt()*b.toInt()
        }
    }

    println("Sum of results: $sum")
}