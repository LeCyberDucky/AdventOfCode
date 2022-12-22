import java.io.File
import kotlin.math.max
@kotlin.ExperimentalStdlibApi

fun main() {
    val input = File("input.txt")
    var sum_of_priorities = 0
    
    input.forEachLine() {
        var compartment_A = BooleanArray('z' - 'A' + 1) { false }
        var compartment_B = BooleanArray('z' - 'A' + 1) { false }
        var rucksack = BooleanArray('z' - 'A' + 1) { false }

        var (items_A, items_B) = it.chunked(it.length/2)

        for (item in items_A.iterator()) {
            compartment_A[item - 'A'] = true
        }

        for (item in items_B.iterator()) {
            compartment_B[item - 'A'] = true
        }

        for (item in 0..rucksack.size-1) {
            rucksack[item] = compartment_A[item] && compartment_B[item]
        }

        for (item in 0..rucksack.size-1) {
            if (rucksack[item]) {
                val letter = (item + 'A'.code).toChar()
                if (letter.isUpperCase()) {
                    sum_of_priorities += letter - 'A' + 26 + 1
                } else {
                    sum_of_priorities += letter - 'a' + 1
                }
            }
        }
    }

     println("Sum of item priorities: $sum_of_priorities")
}