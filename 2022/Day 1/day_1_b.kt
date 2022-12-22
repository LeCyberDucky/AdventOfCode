import java.io.File
import kotlin.math.max
import kotlin.math.min

fun main() {
    val input = File("input.txt")

    var top_elf_calories = arrayListOf(0, 0, 0)
    var current_elf_sum = 0

    input.forEachLine() { 
        if (it.length > 0) {
            current_elf_sum += it.toInt()
        } else {
            val smallest_elf = top_elf_calories.indexOf(top_elf_calories.minOrNull())
            top_elf_calories[smallest_elf] = max(top_elf_calories[smallest_elf], current_elf_sum)
            current_elf_sum = 0
        }
     }

     println("Total calories carried by the biggest three elves: ${top_elf_calories.sum()}")
}