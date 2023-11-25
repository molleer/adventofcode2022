import java.io.File

fun eval(opponent: Int, you: Int): Int {
    if ((opponent + 1) % 3 == you) {
        return you + 7
    } else if (opponent == you) {
        return you + 4
    }
    return you + 1
}

fun getScore(opponent: String, you: String): Int {
    var you_choises = listOf("X", "Y", "Z")
    var opponent_choises = listOf("A", "B", "C")

    var opponent_choise = opponent_choises.indexOf(opponent)
    var you_choise = you_choises.indexOf(you)

    return eval(opponent_choise, you_choise)
}

fun getScore2(opponent: String, you: String): Int {
    var opponent_choises = listOf("A", "B", "C")
    var results = mapOf(Pair("X", -1), Pair("Y", 0), Pair("Z", 1))

    var opponent_choise = opponent_choises.indexOf(opponent)
    var you_choise = (3 + opponent_choise + results.getOrDefault(you, -10)) % 3

    return eval(opponent_choise, you_choise)
}

fun main() {
    var lines = File("input.txt").readLines()
    var score = 0
    for (line in lines) {
        var split = line.split(" ")
        var round_score = getScore2(split[0], split[1])
        score += round_score
    }
    println(score)
}
