# ToeTacTic library

`toetactic_lib` is a library that provides functionality as an engine that plays Tic Tac Toe optimally, on any square grid larger than 2x2 (theoretically).

## Background

**2024.12.06**:

I'd been exploring various algorithms in computer science, and it didn't take long for me to realize that I love learning about these algorithms and understanding them.

Therefore, I decided to put my skills to the test by writing an 'engine' that plays Tic Tac Toe optimally. Now, of course, normal Tic Tac Toe would be too simple. In fact, I myself never lose at Tic Tac Toe. So, I wanted to write my 'engine' such that I can always expand it to larger grids like 5x5. I have a strong feeling that I would suck at 5x5 Tic Tac Toe.

**2024.12.14**:
Over the past week of working on this project, I learned something. My initial statement:

> I have a strong feeling that I would suck at 5x5 Tic Tac Toe.

...was wrong. While playing against my own engine, I realized that the larger the grid, the less the 'sense of unclearness' — the exact opposite of what I'd assumed in the beginning. On a larger grid, it's easy to just fill up a whole row and make it impossible for anyone to win on a column, or fill up a whole column and make it impossible for anyone to win on a row. Same goes for diagonals. What this means is that you can easily sabotage the winning chances of _both_ players. This is why 3x3 Tic Tac Toe is the most popular form and seems to be the most challenging form.
