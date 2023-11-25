import java.io.File

fun getPrio(c: Char): Int {
    if (c.code >= 97) {
        return c.code - 96
    }
    return c.code - 65 + 27
}

fun main() {
    var lines = File("input.txt").readLines()
    var total = 0
    for (line in lines) {
        var sack1 = line.subSequence(0, line.length / 2).asSequence().toSet()
        var sack2 = line.subSequence(line.length / 2, line.length).asSequence().toSet()
        total += getPrio((sack1 - (sack1 - sack2)).first())
    }
    println(total)
    total = 0

    var group = ArrayList<Set<Char>>()
    for (line in lines) {
        group.add(line.asSequence().toSet())
        if (group.size == 3) {
            var common = group[0] - (group[0] - group[1])
            common = common - (common - group[2])
            total += getPrio(common.first())
            group.clear()
        }
    }
    println(total)
}
