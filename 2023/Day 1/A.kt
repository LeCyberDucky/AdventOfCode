import java.io.File

fun main() {
    val input = File("input.txt")
    // val input = File("example input.txt")

    var calibration_values: MutableList<Int> = mutableListOf()

    input.forEachLine() {
        var current_digit = 0
        var first_digit = 0
        var last_digit: Int
        var first_found = false
        it.forEach {
            if (it.isDigit()) {
                current_digit = it.digitToInt()
                if (!first_found) {
                    first_found = true
                    first_digit = current_digit
                }
            }
        }
        last_digit = current_digit
        val calibration_value = (first_digit.toString() + last_digit.toString()).toInt()
        calibration_values.add(calibration_value)
    }

    println(calibration_values.sum())
}
