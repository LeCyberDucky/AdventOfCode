import java.io.File
import kotlin.math.sign
import kotlin.math.abs

fun main() {
    // val input = File("example input.txt")
    val input = File("input.txt")

    var safe_reports = 0
    for (line in input.readLines()) {
        val levels = line.split("\\s+".toRegex()).map(String::toInt)
        var safe_report = true
        val report_sign = sign((levels[0] - levels[1]).toDouble())
        for (i in 1 until levels.size) {
            val a = levels[i - 1]
            val b = levels[i]
            val delta = a - b
            if (!(sign(delta.toDouble()) == report_sign && abs(delta) in 1..3)) {
                safe_report = false
                break
            }
        }
        if (safe_report) {
            safe_reports += 1
        }
    }

    println("Number of safe reports: $safe_reports")
}