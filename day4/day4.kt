package one

import java.io.File

fun parseLine(line: String): List<Int> {
    return line.split(Regex("[-,]")).map { x -> Integer.parseInt(x) }
}

fun main() {
    var lines = File("input.txt").readLines().map { x -> parseLine(x) }
    var count = 0
    for (line in lines) {
        if (line[0] <= line[2] && line[1] >= line[3]) {
            count++
        } else if (line[2] <= line[0] && line[3] >= line[1]) {
            count++
        } else if (line[0] <= line[2] && line[1] >= line[2]) {
            count++
        } else if (line[0] <= line[3] && line[1] >= line[3]) {
            count++
        }
    }
    println(count)
}
