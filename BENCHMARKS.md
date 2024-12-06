# Optimal depths for NxN grids (by trial and error)

This document shows calculation depths for various NxN grid, and the maximum time to find the best move at each depth. Note that these move times vary between positions (and sometimes may be _very_ different). Therefore the values shown are the _usual_ values.

**PS**: For higher depths that take longer to execute, the test may have only been performed two times.

## 3x3

The 3x3 grid is simple, so any depth works. A depth of 9 is ideal because:

- the optimal move is **guaranteed**
- any greater depth wouldn't matter as the game would end

Mathematically, a depth much lower than 9 should also be enough to find the optimal move, but I'm not gonna get into all that.

## 4x4

| Depth | Highest move time/s |         Remarks         |
| :---: | :-----------------: | :---------------------: |
|  11   |         25          |                         |
|  10   |          5          |                         |
|   9   |         3.5         |                         |
|   8   |         0.6         |                         |
|   7   |         0.3         | Moves may be inaccurate |
|   6   |        0.08         | Moves may be inaccurate |
|   5   |        0.04         | Moves may be inaccurate |
|   4   |        0.01         | Moves may be inaccurate |

## 5x5

| Depth | Highest move time/s |         Remarks         |
| :---: | :-----------------: | :---------------------: |
|   8   |         16          |                         |
|   7   |         11          |                         |
|   6   |          1          | Moves may be inaccurate |
|   5   |         0.2         | Moves may be inaccurate |
|   4   |        0.06         | Moves may be inaccurate |

## 6x6

| Depth | Highest move time/s |         Remarks         |
| :---: | :-----------------: | :---------------------: |
|   7   |        22.5         |                         |
|   6   |         5.5         | Moves may be inaccurate |
|   5   |        0.75         | Moves may be inaccurate |
|   4   |         0.2         | Moves may be inaccurate |
|   3   |        0.035        | Moves may be inaccurate |

## 7x7

| Depth | Highest move time/s | Remarks |
| :---: | :-----------------: | :-----: |
|   6   |         24          |         |
|   5   |          2          |         |
|   4   |         0.6         |         |
|   3   |        0.05         |         |
