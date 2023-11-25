import java.io.File

fun main() {
    var lines = File("input.txt").readLines()
    var subSum = 0
    var packins = ArrayList<Int>()

    for (line in lines) {
        if (line.equals("")) {
            packins.add(subSum)
            subSum = 0
        } else {
            subSum += Integer.parseInt(line)
        }
    }

    packins.sort()
    packins.reverse()

    println(packins[0])
    println(packins[0] + packins[1] + packins[2])
}
