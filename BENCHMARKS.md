# Optimal depths for NxN grids (by trial and error)

This document shows calculation depths for various NxN grid, and the maximum time to find the best move at each depth. Note that these move times vary between positions (and sometimes may be _very_ different). Therefore the values shown are the _usual_ values.

**PS**: For higher depths that take longer to execute, the test may not have been performed a sufficient number of times.

**PS #2**: The _Accuracy_ column in each table shows whether or not the algorithm **always** finds the optimal move at that depth. Rows marked with '?' have an unknown accuracy. You're welcome to donate knowledge or computing power in order to try and figure out whether those cells should have '✅' or '❌' in them. Also, just because the algorithm is not always accurate at a certain depth, it doesn't mean that depth is bad. For almost all of these cases, the algorithm performs quite well, and only fails on a small set of cases.

## 3x3

The 3x3 grid is simple, so any depth works. A depth of 9 is ideal because:

- the optimal move is **guaranteed**
- any greater depth wouldn't matter as the game would end

Mathematically, a depth much lower than 9 should also be enough to find the optimal move, but I'm not gonna get into all that.

**UPDATE**: **Depth 6** also guarantees the best move, so there you go.

## 4x4

| Depth | Highest move time/s | Accuracy |
| :---: | :-----------------: | :------: |
|  11   |         4.6         |    ?     |
|  10   |         2.8         |    ?     |
|   9   |        0.96         |    ?     |
|   8   |        0.53         |    ?     |
|   7   |        0.15         |    ?     |
|   6   |        0.075        |    ?     |
|   5   |        0.02         |    ❌    |
|   4   |        0.01         |    ❌    |

## 5x5

| Depth | Highest move time/s | Accuracy |
| :---: | :-----------------: | :------: |
|   8   |         17          |    ?     |
|   7   |         2.6         |    ?     |
|   6   |         1.2         |    ?     |
|   5   |         0.2         |    ❌    |
|   4   |        0.062        |    ❌    |
|   3   |        0.01         |    ❌    |

## 6x6

| Depth | Highest move time/s | Accuracy |
| :---: | :-----------------: | :------: |
|   7   |        14.7         |    ?     |
|   6   |         5.5         |    ?     |
|   5   |        0.49         |    ❌    |
|   4   |        0.192        |    ❌    |
|   3   |        0.025        |    ❌    |

## 7x7

| Depth | Highest move time/s | Accuracy |
| :---: | :-----------------: | :------: |
|   6   |        21.9         |    ?     |
|   5   |        1.62         |    ❌    |
|   4   |        0.531        |    ❌    |
|   3   |        0.05         |    ❌    |
