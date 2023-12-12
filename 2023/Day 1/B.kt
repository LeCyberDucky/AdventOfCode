import java.io.File

fun main() {
    val input = File("input.txt")
    // val input = File("example input - B.txt")
    
    var calibration_values: MutableList<Int> = mutableListOf()
    
    input.forEachLine() {
        // Check numeric digits first
        var current_digit = Pair(0, -1)
        var first_digit = Pair(it.length + 1, -1)
        var last_digit: Pair<Int, Int>
        var first_found = false
        for ((i, letter) in it.withIndex()) {
            if (letter.isDigit()) {
                current_digit = Pair(i, letter.digitToInt())
                if (!first_found) {
                    first_found = true
                    first_digit = current_digit
                }
            }
        }
        last_digit = current_digit

        // Check if textual digits occur earlier or later than numerical digits
        val word_occurences = number_word_occurences(it)
        for ((i, word) in word_occurences.withIndex()) {
            if (word.first > -1 && word.first < first_digit.first) {
                first_digit = Pair(word.first, i)
            }
            
            if (word.second > -1 && word.second > last_digit.first) {
                last_digit = Pair(word.second, i)
            }
        }
        
        // Construct and store calibration value
        val calibration_value = (first_digit.second.toString() + last_digit.second.toString()).toInt()
        calibration_values.add(calibration_value)
    }
    
    println(calibration_values.sum())
}

fun number_word_occurences(line: String): MutableList<Pair<Int, Int>> {
    // For all words, find their first occurence both forward and reversed (= last occurence)
    val numbers = listOf("zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine")
    val occurences = MutableList(10) { Pair(-1, -1) }
    for ((i, number) in numbers.withIndex()) {
        val first_occurence = line.indexOf(number, ignoreCase = true)
        var last_occurence = line.reversed().indexOf(number.reversed(), ignoreCase = true)
        if (last_occurence > -1) {
            last_occurence = line.length - (last_occurence + number.length)
        }
        occurences[i] = Pair(first_occurence, last_occurence)
    }
    return occurences
}