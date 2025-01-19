# n-puzzle

## Some example puzzles

- [n-puzzle](#n-puzzle)
  - [Some example puzzles](#some-example-puzzles)
    - [A simple 8-puzzle example](#a-simple-8-puzzle-example)
    - [A simple 15-puzzle example](#a-simple-15-puzzle-example)
    - [15-puzzle example from Wikipedia](#15-puzzle-example-from-wikipedia)
    - [15-puzzle example from `instructables.com`](#15-puzzle-example-from-instructablescom)

### A simple 8-puzzle example

This should be solvable pretty quickly with everything except for possibly DFS.

```text
7 8 5
3 1 4
6 2 _
```

You can run this with

```bash
cargo run --release -- --pieces 7,8,5,3,1,4,6,2 --x-blank 0 --y-blank 2
```

### A simple 15-puzzle example

This is a simple 15-puzzle example where the first 11 tiles are in the correct
position. It only takes 12 moves to solve. You can find solutions in a reasonable
amount of time breadth-first search, but depth-first search still tends to get lost.

```text
 1  2  3  4
 5  6  7  8
 9 10 11 15
13 12 14  _
```

You can run this with

```bash
cargo run --release -- --pieces 1,2,3,4,5,6,7,8,9,10,11,15,13,12,14 --x-blank 3 --y-blank 3
```

### 15-puzzle example from Wikipedia

[The Wikipedia page for the 15-puzzle](https://en.wikipedia.org/wiki/15_puzzle) has an image of this example.
This takes 49 steps to solve, so anything except A* with taxicab (or better) as the heuristic is likely to take quite a while.

```text
12  1  2 15
11  6  5  8
 7 10  9  4
_  13 14  3
```

You can run this with

```bash
cargo run --release -- --pieces 12,1,2,15,11,6,5,8,7,10,9,4,13,14,3 --x-blank 3 --y-blank 0
```

### 15-puzzle example from `instructables.com`

This example is from [the `instructables.com` article "How to solve the 15 puzzle"](https://www.instructables.com/How-To-Solve-The-15-Puzzle/)
after the 1 tile has been moved into the correct position, and the 2 tile has been partially placed.
This takes 58 moves to solve, so it's likely to take quite a while if you're using anything
except for A* with the taxicab heuristic (or something better).

```text
 1  5 10  9
15  _  4 14
12  2  8 13
11  7  3  6
```

You can run this with

```bash
cargo run --release -- --pieces 1,5,10,9,15,4,14,12,2,8,13,11,7,3,6 --x-blank 1 --y-blank 1
```
