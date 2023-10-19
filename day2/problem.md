https://adventofcode.com/2022/day/2

--------------------PART 1 --------------------------

Puzzle input - encrypted strategy guide 
- The first column is what your opponent is going to play
    A for Rock, B for Paper, and C for Scissors.
- The second column-- ??
    X for Rock, Y for Paper, and Z for Scissors.

Puzzle Output: calculate the score you would get if you were to follow the strategy guide.


### Context:

Among Elves, a giant Rock Paper Scissors tournament is in progress.
Rock Paper Scissors is a game between two players. Each game contains many rounds; in each round, the players each simultaneously choose one of Rock, Paper, or Scissors using a hand shape. Then, a winner for that round is selected: Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock. If both players choose the same shape, the round instead ends in a draw.

The winner of the whole tournament is the player with the highest score.

Your total score is the sum of your scores for each round
Score of one round = (shape you selected) + (outcome of the round)
- shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors)
- outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won)

--- Part Two --------------------------

"Anyway, the second column says how the round needs to end: X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win. Good luck!"

The total score is still calculated in the same way, but now you need to figure out what shape to choose so the round ends as indicated.

Following the Elf's instructions for the second column, what would your total score be if everything goes exactly according to your strategy guide?