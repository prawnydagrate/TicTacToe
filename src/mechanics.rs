/// Represents a cell on the grid.
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

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Game {
    grid: Grid,
    empty: Vec<(usize, usize)>,
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
    /// to a `Vec<(usize, usize)>`.
    pub fn empty(&self) -> &Vec<(usize, usize)> {
        &self.empty
    }

    /// Attempts to play `X` or `O` (depending on which
    /// player's turn it is to move) in the given position.
    /// Requirements:
    /// * The game must be ongoing
    /// * The position (`row`, `col`) must be within the grid,
    /// and empty
    pub fn play(&mut self, row: usize, col: usize) -> Result<(), ()> {
        let n = self.grid.n();
        if self.state != GameState::Ongoing
            || row >= n
            || col >= n
            || self.grid.data()[row][col] != Cell::Empty
        {
            return Err(());
        }
        self.grid.data_mut()[row][col] = self.turn.into();
        self.update_state();
        Ok(())
    }

    fn update_state(&mut self) {
        let n = self.grid.n();

        let xwin = n as isize;
        let owin = -xwin;

        let mut cols = vec![Vec::new(); n];
        let mut diags = vec![Vec::new(); 2];

        let mut empty = Vec::new();

        // check rows
        for (i, row) in self.grid.data().iter().enumerate() {
            let row_score: isize = row
                .iter()
                .enumerate()
                .map(|(j, &cell)| {
                    let cell_score: isize = cell.into();
                    if cell == Cell::Empty {
                        empty.push((i, j));
                    }
                    if i == j {
                        diags[0].push(cell_score);
                    }
                    if i + j == n - 1 {
                        diags[1].push(cell_score);
                    }
                    cols[i].push(cell_score);
                    cell_score
                })
                .sum();
            if row_score == xwin {
                return self.state = GameState::Decisive(Player::X);
            }
            if row_score == owin {
                return self.state = GameState::Decisive(Player::O);
            }
        }
        // check columns
        for col in cols {
            let col_score: isize = col.into_iter().sum();
            if col_score == xwin {
                return self.state = GameState::Decisive(Player::X);
            }
            if col_score == owin {
                return self.state = GameState::Decisive(Player::O);
            }
        }
        // check diagonals
        for diag in diags {
            let diag_score: isize = diag.into_iter().sum();
            if diag_score == xwin {
                return self.state = GameState::Decisive(Player::X);
            }
            if diag_score == owin {
                return self.state = GameState::Decisive(Player::O);
            }
        }

        self.state = if empty.is_empty() {
            GameState::Tied
        } else {
            GameState::Ongoing
        }
    }
}
