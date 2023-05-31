# chess

Hobby project to implement game of chess.
Focus on AI and target is that it plays better than its creator.

````
  1   -5   -2   -3  -99   -9   -3   -2   -5
  2   -1   -1   -1    0    0   -1   -1   -1
  3    0    0    0   -1    0    0    0    0
  4    0    0    0    0   -1    0    0    0
  5    0    0    3    0    1    0    0    0
  6    0    0    2    0    0    0    0    0
  7    1    1    1    1    0    1    1    1
  8    5    0    3   99    9    0    2    5

       A    B    C    D    E    F    G    H
````

## TODO

* [x] Support long movement notation
* [x] Render board to terminal
* [x] Out of bounds check
* [ ] Movement rules
* [ ] Game loop
* [ ] Support shorter movement notation
* [ ] Detect check and checkmate
* [ ] Add AI player
* [ ] Castling
* [ ] En passant

## How to run

`cargo run` - Renders board to terminal

`cargo test` - Run unit tests
