//! Game mechanics

use std::fmt::{self, Formatter};

/// Represents the contents of a cell on the grid.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Cell {
    X,
    O,
    Empty,
}

impl From<Cell> for isize {
    fn from(cell: Cell) -> Self {
        match cell {
            Cell::X => 1,
            Cell::O => -1,
            Cell::Empty => 0,
        }
    }
}

/// Represents a player, the player playing X
/// or the player playing O.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Player {
    X,
    O,
}

impl From<Player> for isize {
    fn from(player: Player) -> Self {
        match player {
            Player::X => 1,
            Player::O => -1,
        }
    }
}

impl From<Player> for Cell {
    fn from(player: Player) -> Self {
        match player {
            Player::X => Cell::X,
            Player::O => Cell::O,
        }
    }
}

impl std::ops::Not for Player {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::X => Self::O,
            Self::O => Self::X,
        }
    }
}

/// Represents the game state.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum GameState {
    Ongoing,
    Tied,
    /// The tuple field gives the player whom won.
    Decisive(Player),
}

/// Represents the game's grid data as a [`Vec`] of rows, where each row is a [`Vec`] of cells.
pub type GridData = Vec<Vec<Cell>>;

/// Represents the game grid.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Grid(GridData);

impl Grid {
    /// Creates an empty `n` by `n` square grid.
    /// **Panics** if `n` < 3.
    pub fn new(n: usize) -> Self {
        assert!(n > 2);
        Self(vec![vec![Cell::Empty; n]; n])
    }

    /// Returns a reference to the grid data.
    pub fn data(&self) -> &GridData {
        &self.0
    }

    /// Returns a mutable reference to the grid data.
    fn data_mut(&mut self) -> &mut GridData {
        &mut self.0
    }

    /// Returns the dimensions of the `n` by `n` grid.
    pub fn n(&self) -> usize {
        self.0.len()
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let n = self.n();
        let mut pp = String::new();
        pp.push_str(
            format!(
                "┌{}┐\n",
                "───┬".repeat(n).chars().take(4 * n - 1).collect::<String>()
            )
            .as_str(),
        );
        for (i, row) in self.0.iter().enumerate() {
            for (j, &col) in row.iter().enumerate() {
                pp.push_str(
                    format!(
                        "│ {} ",
                        match col {
                            Cell::X => 'X',
                            Cell::O => 'O',
                            Cell::Empty => ' ',
                        }
                    )
                    .as_str(),
                );
                if j == n - 1 {
                    pp.push_str("│\n");
                }
            }
            if i != n - 1 {
                pp.push_str(
                    format!(
                        "├{}┤\n",
                        "───┼".repeat(n).chars().take(4 * n - 1).collect::<String>()
                    )
                    .as_str(),
                );
            }
        }
        pp.push_str(
            format!(
                "└{}┘",
                "───┴".repeat(n).chars().take(4 * n - 1).collect::<String>()
            )
            .as_str(),
        );
        write!(f, "{pp}")
    }
}

/// Represents a move as a row index (first) and a column index (second) on a grid.
pub type Move = (usize, usize);

/// Represents a game of Tic Tac Toe.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Game {
    grid: Grid,
    empty: Vec<Move>,
    undoubted: Option<(Player, Move)>,
    state: GameState,
    turn: Player,
}

impl Game {
    /// Creates a new game with an `n` by `n` grid.
    /// **Panics** if n < 3.
    pub fn new(n: usize) -> Self {
        assert!(n > 2);
        let mut empty = Vec::new();
        for i in 0..n {
            for j in 0..n {
                empty.push((i, j));
            }
        }
        Self {
            grid: Grid::new(n),
            empty,
            undoubted: None,
            state: GameState::Ongoing,
            turn: Player::X,
        }
    }

    /// Returns a reference to the game grid.
    pub fn grid(&self) -> &Grid {
        &self.grid
    }

    /// Returns the player whose turn it is to move.
    pub fn turn(&self) -> Player {
        self.turn
    }

    /// Returns the game state.
    pub fn state(&self) -> GameState {
        self.state
    }

    /// Returns the positions of the remaining empty
    /// cells in which a move may be played, as a reference
    /// to a `Vec<Move>`.
    pub fn empty(&self) -> &Vec<Move> {
        &self.empty
    }

    /// Returns the 'obvious move' in the position, if there
    /// is such a move (i.e., a chance to immediately win
    /// or prevent the opponent's immediate win, with an
    /// immediate win given higher priority). Along with the
    /// obvious move, the function returns the side who wins
    /// with this move or would win if not for this move.
    pub fn undoubted(&self) -> Option<(Player, Move)> {
        self.undoubted
    }

    /// Attempts to play `X` or `O` (depending on which
    /// player's turn it is to move) in the given position.
    /// This function returns `None` if the move was unsuccessful.
    /// Requirements:
    /// * The game must be ongoing
    /// * The position (`row`, `col`) must be within the grid,
    ///   and empty
    pub fn play(&mut self, mv: Move) -> Option<()> {
        let (row, col) = mv;
        let n = self.grid.n();
        if self.state != GameState::Ongoing
            || row >= n
            || col >= n
            || self.grid.data()[row][col] != Cell::Empty
        {
            return None;
        }
        self.grid.data_mut()[row][col] = self.turn.into();
        self.turn = !self.turn;
        self.update_state();
        Some(())
    }

    fn update_state(&mut self) {
        self.undoubted = None;

        let n = self.grid.n();

        let xw = n as isize;
        let ow = -xw;

        let mut row_scores = Vec::new();
        let mut cols = vec![Vec::new(); n];
        let mut diags = vec![Vec::new(); 2];

        let mut empty = Vec::new();
        let (mut xwin, mut owin) = (None, None);

        // check rows
        for (i, row) in self.grid.data().iter().enumerate() {
            let mut nempty = 0;
            let mut empty_cell = None;
            let row_score: isize = row
                .iter()
                .enumerate()
                .map(|(j, &cell)| {
                    let pos = (i, j);
                    if cell == Cell::Empty {
                        nempty += 1;
                        empty_cell = Some(pos);
                        empty.push(pos);
                    }
                    let cell_score: isize = cell.into();
                    // it's in the \ diagonal
                    if i == j {
                        diags[0].push((cell_score, pos));
                    }
                    // it's in the / diagonal
                    if i + j == n - 1 {
                        diags[1].push((cell_score, pos));
                    }
                    cols[j].push((cell_score, pos));
                    cell_score
                })
                .sum();
            row_scores.push((nempty, row_score, empty_cell));
        }
        self.empty = empty;
        for (nempty, row_score, empty_cell) in row_scores {
            if row_score == xw {
                return self.state = GameState::Decisive(Player::X);
            } else if row_score == ow {
                return self.state = GameState::Decisive(Player::O);
            }
            if nempty == 1 {
                if row_score == xw - 1 {
                    xwin = empty_cell
                } else if row_score == ow + 1 {
                    owin = empty_cell;
                }
            }
        }
        // check columns
        for col in cols {
            let mut nempty = 0;
            let mut empty_cell = None;
            let col_score: isize = col
                .into_iter()
                .map(|(cell_score, pos)| {
                    if cell_score == 0 {
                        nempty += 1;
                        empty_cell = Some(pos);
                    }
                    cell_score
                })
                .sum();
            if col_score == xw {
                return self.state = GameState::Decisive(Player::X);
            }
            if col_score == ow {
                return self.state = GameState::Decisive(Player::O);
            }
            if nempty == 1 {
                if col_score == xw - 1 {
                    xwin = empty_cell;
                } else if col_score == ow + 1 {
                    owin = empty_cell;
                }
            }
        }
        // check diagonals
        for diag in diags {
            let mut nempty = 0;
            let mut empty_cell = None;
            let diag_score: isize = diag
                .into_iter()
                .map(|(cell_score, pos)| {
                    if cell_score == 0 {
                        nempty += 1;
                        empty_cell = Some(pos);
                    }
                    cell_score
                })
                .sum();
            if diag_score == xw {
                return self.state = GameState::Decisive(Player::X);
            }
            if diag_score == ow {
                return self.state = GameState::Decisive(Player::O);
            }
            if nempty == 1 {
                if diag_score == xw - 1 {
                    xwin = empty_cell;
                } else if diag_score == ow + 1 {
                    owin = empty_cell;
                }
            }
        }
        // check if there are any obvious moves
        let (xwin, owin) = (
            xwin.map(|pos| (Player::X, pos)),
            owin.map(|pos| (Player::O, pos)),
        );
        self.undoubted = if self.turn == Player::X && xwin.is_some() {
            xwin
        } else if self.turn == Player::O && owin.is_some() {
            owin
        } else if xwin.is_some() {
            xwin
        } else if owin.is_some() {
            owin
        } else {
            None
        };

        self.state = if self.empty.is_empty() {
            GameState::Tied
        } else {
            GameState::Ongoing
        }
    }
}
