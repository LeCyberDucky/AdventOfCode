import java.io.File
import kotlin.math.max
@kotlin.ExperimentalStdlibApi

fun main() {
    val input = File("input.txt")
    var sum_of_priorities = 0
    var elf = 0

    var group_rucksacks = Array(3) {BooleanArray('z' - 'A' + 1) { false }}
    
    input.forEachLine() {
        for (item in it.iterator()) {
            group_rucksacks[elf][item - 'A'] = true
        }

        if (elf == 2) {
            // Group completed
            var rucksack_overlap = BooleanArray('z' - 'A' + 1) { false }
            for (item in 0..rucksack_overlap.size-1) {
                rucksack_overlap[item] = group_rucksacks[0][item] && group_rucksacks[1][item] && group_rucksacks[2][item]
            }
            
            for (item in 0..rucksack_overlap.size-1) {
                if (rucksack_overlap[item]) {
                    val letter = (item + 'A'.code).toChar()
                    if (letter.isUpperCase()) {
                        sum_of_priorities += letter - 'A' + 26 + 1
                    } else {
                        sum_of_priorities += letter - 'a' + 1
                    }
                }
            }
            group_rucksacks = Array(3) {BooleanArray('z' - 'A' + 1) { false }}
        }
        elf = (elf + 1) % 3
    }

     println("Sum of item priorities: $sum_of_priorities")
}