## Conway's Game of life 
It is a fascinatingly simple game but can yeld deceptively complex results (it is turing complete). 

The most common ruleset is:
    1. Any live cell with fewer than two live neighbours dies, as if by underpopulation.
    2. Any live cell with two or three live neighbours lives on to the next generation.
    3. Any live cell with more than three live neighbours dies, as if by overpopulation.
    4. Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.

  (https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life)

## This project
This project is an attempt at getting a more performant version than my old python program. It also serves as a nice exercise in learning how to make a CLI in rust. 
