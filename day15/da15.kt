import java.io.File
import kotlin.math.abs
import kotlin.math.max
import kotlin.math.min

fun man_dist(a: Pair<Int, Int>, b: Pair<Int, Int>): Int {
    return abs(a.component1() - b.component1()) + abs(a.component2() - b.component2())
}

fun get_non_beacon(y: Int, sensor: Pair<Int, Int>, beacon: Pair<Int, Int>): Pair<Int, Int> {
    val man = man_dist(sensor, beacon)
    var diff = abs(man - abs(sensor.component2() - y))
    if (abs(sensor.component2() - y) > man) {
        return Pair(1, 0)
    }
    return Pair((sensor.component1() - diff), (sensor.component1() + diff))
}

fun parse_line(line: String): Pair<Pair<Int, Int>, Pair<Int, Int>> {
    var split = line.removePrefix("Sensor at x=").split(": closest beacon is at x=")
    var sensor = split[0].split(", y=")
    var beacon = split[1].split(", y=")
    return Pair(
            Pair(Integer.parseInt(sensor[0]), Integer.parseInt(sensor[1])),
            Pair(Integer.parseInt(beacon[0]), Integer.parseInt(beacon[1]))
    )
}

fun missing_at(ranges: List<Pair<Int, Int>>): Int {
    var sortedRange = ranges.sortedBy { it.component1() }
    var largest_in_range = 0
    for (range in sortedRange) {
        if (range.component1() > largest_in_range + 1) {
            return largest_in_range + 1
        }
        largest_in_range = max(largest_in_range, range.component2())
    }
    return -1
}

fun main1() {
    var lines = File("input.txt").readLines()
    val y = 20
    var beacon_x = HashSet<Int>()
    var non_beacon_x = HashSet<Int>()

    for (line in lines) {
        var pos = parse_line(line)
        val non_x = get_non_beacon(y, pos.component1(), pos.component2())

        if (pos.component2().component2() == y) {
            beacon_x.add(pos.component2().component1())
        }
        non_beacon_x.addAll(non_x.component1()..non_x.component2())
    }
    non_beacon_x.removeAll(beacon_x)
    println(non_beacon_x.size)
}

fun main2() {
    var lines = File("input.txt").readLines()
    val max_x = 4_000_000
    var non_beacon_x = (0..max_x).map { ArrayList<Pair<Int, Int>>() }

    for (line in lines) {
        var pos = parse_line(line)
        var man = man_dist(pos.component1(), pos.component2())
        for (y in
                max(0, pos.component1().component2() - man)..min(
                                max_x,
                                pos.component1().component2() + man
                        )) {
            val non_x = get_non_beacon(y, pos.component1(), pos.component2())
            non_beacon_x[y].add(non_x)
        }
    }
    for (y in 0..max_x) {
        val x = missing_at(non_beacon_x[y])
        if (x != -1) {
            val f = x * 4_000_000L + y
            println("$f")
            break
        }
    }
}
