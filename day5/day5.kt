import java.io.File
import kotlin.streams.asSequence

fun main() {
    var lines = File("input.txt").readLines()
    var n_stacks = (lines[0].length + 1) / 4
    var stacks = (0 ..< n_stacks).map { _ -> ArrayList<Char>() }

    var k = 0
    for (line in lines) {
        var chars = line.asSequence().toList()
        if (chars[1] == '1') {
            break
        }
        k++

        for (i in 0 ..< n_stacks) {
            var name = chars[1 + i * 4]
            if (name != ' ') {
                stacks.get(i).add(name)
            }
        }
    }

    for (line in lines.subList(k + 2, lines.size)) {
        var split = line.split(" ")
        var from = Integer.parseInt(split[3]) - 1
        var to = Integer.parseInt(split[5]) - 1
        for (i in Integer.parseInt(split[1]) - 1 downTo 0) {
            stacks[to].add(0, stacks[from][i])
            stacks[from].removeAt(i)
        }
    }
    println(stacks.stream().map { x -> x.first() }.asSequence().toList())
}
