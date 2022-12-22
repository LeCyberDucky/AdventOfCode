import java.io.File
import kotlin.math.max

fun main() {
    val input = File("input.txt")

    var max_calories = 0
    var current_elf_sum = 0

    input.forEachLine() { 
        if (it.length > 0) {
            current_elf_sum += it.toInt()
        } else {
            max_calories = max(max_calories, current_elf_sum)
            current_elf_sum = 0
        }
     }

     println("Total calories carried by the biggest elf: $max_calories")
}