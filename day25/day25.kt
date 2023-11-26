import java.io.File

fun toBase10(snafu: String): Long {
    var chars = listOf('=', '-', '0', '1', '2')
    var base10 = 0L
    var i = 1L
    for (digit in snafu.reversed()) {
        base10 += (chars.indexOf(digit) - 2) * i
        i *= 5L
    }
    return base10
}

fun toBase5(base10: Int): String {
    return Integer.toString(base10, 5)
}

fun toSNAFU(base10: Long): String {
    return toSNAFU_helper(base10, 0)
}

fun pow(n: Long, expo: Long): Long {
    var res = 1L
    for (i in 0 ..< expo) {
        res *= n
    }
    return res
}

fun toSNAFU_helper(n: Long, exp: Long): String {
    if (n == 0L) {
        return ""
    }
    var signs = listOf('=', '-', '0', '1', '2')
    var id = (n / pow(5, exp) + 2) % 5
    return toSNAFU_helper(n - pow(5, exp) * (id - 2), exp + 1) + signs[id.toInt()]
}

fun main() {
    var lines = File("input.txt").readLines()

    var sum = 0L
    for (line in lines) {
        sum += toBase10(line)
    }
    println(sum)
    println(toSNAFU(sum))
}
