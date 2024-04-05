use rand::{seq::SliceRandom, thread_rng, Rng};
use std::{
    collections::HashSet,
    fmt::{self, Display},
    str::FromStr,
};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Cache {
    row_cache: (u8, u8, u8),
    col_cache: (u8, u8, u8),
    box_cache: (u8, u8, u8),
}

impl Cache {
    /// Generate the cache. For each entry in the sudoku board, there
    /// will be three entries in the cache for uniqueness in row, col
    /// and a box. For row uniqueness, the entry is (row, cell_val, dim+1),
    /// (dim+1, cell_val, col) for column uniqueness, and for box uniqueness
    /// we use (row / box_size, cell_val, col / box_size)
    fn new(r: u8, c: u8, cell_val: u8, dim: u8, box_size: u8) -> Self {
        Cache {
            row_cache: (r, cell_val, dim + 1),  // for unique row
            col_cache: (dim + 1, cell_val, c),  // for unique col
            box_cache: (r / box_size, cell_val, c / box_size),  // for unique box
        }
    }
}

#[derive(Debug, Clone)]
pub enum SudokuParseError {
    InvalidFormat,
    InvalidCharacter,
}

impl Display for SudokuParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SudokuParseError::InvalidFormat => write!(
                f,
                "Invalid format: the input does not match the expected format."
            ),
            SudokuParseError::InvalidCharacter => write!(
                f,
                "Invalid character: the input contains an invalid character."
            ),
        }
    }
}

impl PartialEq for SudokuParseError {
    fn eq(&self, _othr: &SudokuParseError) -> bool {
        true
    }
}

impl Eq for SudokuParseError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sudoku {
    // board to store the sudoku values
    board: Vec<Vec<u8>>,
}

impl Sudoku {
    fn new(dim: usize) -> Self {
        if dim != 9 {
            panic!("Currently only 9x9 sudoku config is tested and supported :(");
        }
        Sudoku {
            board: vec![vec![0; dim]; dim],
        }
    }

    fn dims(&self) -> usize {
        self.board.len()
    }

    fn get_board_mut(&mut self) -> &mut Vec<Vec<u8>> {
        &mut self.board
    }

    fn set_value(&mut self, r: usize, c: usize, val: u8) {
        self.board[r][c] = val;
    }

    fn get_value(&self, r: usize, c: usize) -> u8 {
        self.board[r][c]
    }
    /// 9x9 : box = 3x3 (9/3 = 3)
    /// 4x4 : box = 2x2 (4/2 = 2)
    /// 16x16 : box = 4x4 (16/4 = 4)
    /// 2x2 : box = 1x1 (2/2 = 1) --> Anamoly
    /// Gets the box size in the sudoku board
    fn find_box_size(&self) -> u8 {
        (self.dims() as f32).sqrt() as u8
    }

    /// Create a lookup for row, col and box values as Cache
    fn make_cache(&self, r: usize, c: usize, cell_val: u8) -> Cache {
        Cache::new(
            r as u8,
            c as u8,
            cell_val,
            self.dims() as u8,
            self.find_box_size(),
        )
    }
    /// Create a lookup table for possible values
    fn make_lk_up(&self) -> HashSet<(u8, u8, u8)> {
        let mut lk_up: HashSet<(u8, u8, u8)> = HashSet::new();
        for r in 0..self.dims() {
            for c in 0..self.dims() {
                if self.get_value(r, c) != 0 {
                    let cache = self.make_cache(r, c, self.get_value(r, c));
                    if lk_up.contains(&cache.row_cache)
                        || lk_up.contains(&cache.col_cache)
                        || lk_up.contains(&cache.box_cache)
                    {
                        panic!(
                            "{} already present, found at ({}, {})",
                            self.get_value(r, c),
                            r,
                            c
                        );
                    }
                    lk_up.insert(cache.row_cache);
                    lk_up.insert(cache.col_cache);
                    lk_up.insert(cache.box_cache);
                }
            }
        }
        lk_up
    }

    /// Fills a blank board (all 0's) with valid numbers at random positions
    /// using recursion and backtracking.
    fn fill_board(&mut self, lk_up: &mut HashSet<(u8, u8, u8)>, nums: &mut Vec<u8>) -> bool {
        for row in 0..self.dims() {
            for col in 0..self.dims() {
                if self.get_value(row, col) == 0 {
                    // randomize the entries at each position
                    nums.shuffle(&mut thread_rng());

                    for i in 0..self.dims() {
                        let cell_val = nums[i];

                        if self.is_valid(&lk_up, cell_val, row, col) {
                            self.set_value(row, col, cell_val);

                            let cache = self.make_cache(row, col, self.get_value(row, col));

                            lk_up.insert(cache.row_cache);
                            lk_up.insert(cache.col_cache);
                            lk_up.insert(cache.box_cache);

                            if self.is_filled() {
                                return true;
                            }

                            if self.fill_board(lk_up, nums) {
                                return true;
                            }

                            lk_up.remove(&cache.row_cache);
                            lk_up.remove(&cache.col_cache);
                            lk_up.remove(&cache.box_cache);
                        }
                    }
                    self.set_value(row, col, 0);
                    return false;
                }
            }
        }
        true
    }

    /// Check if the cell_val at row & col is a valid entry
    /// return true if valid, else false.
    fn is_valid(
        &self,
        lk_up: &HashSet<(u8, u8, u8)>,
        cell_val: u8,
        row: usize,
        col: usize,
    ) -> bool {
        let cache = self.make_cache(row, col, cell_val);

        !lk_up.contains(&cache.row_cache)
            && !lk_up.contains(&cache.col_cache)
            && !lk_up.contains(&cache.box_cache)
    }

    /// Check if the board's all cells are filled with all. Return true
    /// if all cells are filled else false.
    fn is_filled(&self) -> bool {
        self.board
            .iter()
            .all(|row| row.iter().all(|&cell| cell != 0))
    }
}

impl FromStr for Sudoku {
    type Err = SudokuParseError;

    /// Input will be of form:
    ///  -------------------
    /// "|0 0 0|0 0 0|0 0 0|"
    /// "|0 0 0|0 0 0|0 0 0|"
    /// "|0 0 0|0 0 0|0 0 0|"
    ///  -------------------
    /// "|0 0 0|0 0 0|0 0 0|"
    /// "|0 0 0|0 0 0|0 0 0|"
    /// "|0 0 0|0 0 0|0 0 0|"
    ///  -------------------
    /// "|0 0 0|0 0 0|0 0 0|"
    /// "|0 0 0|0 0 0|0 0 0|"
    /// "|0 0 0|0 0 0|0 0 0|"
    ///  -------------------
    /// Parse a string representation of the sudoku puzzle into Sudoku config.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut game = Sudoku::new(9);
        let board = game.get_board_mut();
        let mut line_counter = 0;

        let rows = s.lines().skip(1).filter(|line| !line.trim().is_empty());
        let mut idx = 0;
        for line in rows {
            if line_counter == 0 || line_counter == 4 || line_counter == 8 || line_counter == 12 {
                // Skip lines with dashes
                line_counter += 1;
                continue;
            }

            let cells: Vec<u8> = line
                .chars()
                .filter(|&c| c != '|' && c != ' ')
                .map(|c| {
                    c.to_digit(10)
                        .ok_or(SudokuParseError::InvalidCharacter)
                        .and_then(|d| {
                            if d <= 9 {
                                Ok(d as u8)
                            } else {
                                Err(SudokuParseError::InvalidCharacter)
                            }
                        })
                })
                .collect::<Result<Vec<u8>, SudokuParseError>>()?;

            if cells.len() != 9 {
                return Err(SudokuParseError::InvalidFormat);
            }

            board[idx] = cells;
            idx += 1;
            line_counter += 1;
        }

        if board.len() != 9 {
            return Err(SudokuParseError::InvalidFormat);
        }

        Ok(game)
    }
}

impl Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, row) in self.board.iter().enumerate() {
            if i % 3 == 0 && i != 0 {
                // Add horizontal separator after every 3 rows
                writeln!(f, "---------------------")?;
            }
            for (j, &cell) in row.iter().enumerate() {
                if j % 3 == 0 && j != 0 {
                    // Add vertical separator after every 3 cells
                    write!(f, "| ")?;
                }
                write!(f, "{} ", cell)?;
            }
            // Newline after each row
            writeln!(f)?;
        }
        Ok(())
    }
}
/// Check if the numbers that are populated in a given puzzle are valid
pub fn valid_board(game: &Sudoku) -> bool {
    let lk_up = game.make_lk_up();

    lk_up.len() == game.dims() * game.dims() * 3
}

/// Helper function that goes through the board and if the value is 0 
/// at a position, it tries to populate it with a valid number. If 
/// subsequent recursive calls fail to fill the board completly, it
/// tries a different value in backtracking fashion.
fn solve_puzzle_util(
    game: &mut Sudoku,
    lk_up: &mut HashSet<(u8, u8, u8)>,
    nums: &mut Vec<u8>,
) -> bool {
    for row in 0..game.dims() {
        for col in 0..game.dims() {
            if game.get_value(row, col) == 0 {
                for i in 0..game.dims() {
                    let cell_val = nums[i];

                    if game.is_valid(&lk_up, cell_val, row, col) {
                        game.set_value(row, col, cell_val);

                        let cache = game.make_cache(row, col, game.get_value(row, col));

                        lk_up.insert(cache.row_cache);
                        lk_up.insert(cache.col_cache);
                        lk_up.insert(cache.box_cache);

                        if game.is_filled() {
                            return true;
                        }

                        if solve_puzzle_util(game, lk_up, nums) {
                            return true;
                        }

                        lk_up.remove(&cache.row_cache);
                        lk_up.remove(&cache.col_cache);
                        lk_up.remove(&cache.box_cache);
                    }
                }
                game.set_value(row, col, 0);
                return false;
            }
        }
    }
    true
}

/// Solve the puzzle
/// Create a lookup table for all possible numbers and
/// use a recursive helper function to populate the puzzle.
pub fn solve_puzzle(game: &mut Sudoku) {
    let mut nums: Vec<u8> = (1..(game.dims() + 1) as u8).collect();

    let mut lk_up = game.make_lk_up();

    solve_puzzle_util(game, &mut lk_up, &mut nums);
}

/// Find out if the board has unique solution
/// Returns true if the puzzle has unique solution; false otherwise
/// Calls the solve_board for all valid numbers that can be populated at the given (row,col)
pub fn unique_soln(game: &Sudoku, row: usize, col: usize) -> bool {
    let lk_up = game.make_lk_up();

    let mut found: bool = false;

    for val in 1..game.dims() as u8 {
        if game.is_valid(&lk_up, val, row, col) {
            let mut board_search: Sudoku = game.clone();
            board_search.set_value(row, col, val);
            solve_puzzle(&mut board_search);

            if board_search.is_filled() {
                if found {  // There was already a solution present
                    return false;
                }
                found = true;
            }
        }
    }
    true
}

/// Generate a random sudoku board of dim x dim config
/// Guaranteed to have unique solution
/// Returns a Sudoku populated with partial solution
/// Usage: let board = gen_board(9);
pub fn gen_board(dim: usize) -> Sudoku {
    // initialize a board of 9x9 with all cells 0
    let mut game = Sudoku::new(dim);
    let mut nums: Vec<u8> = (1..(dim + 1) as u8).collect();
    let mut lk_up: HashSet<(u8, u8, u8)> = HashSet::new();

    game.fill_board(&mut lk_up, &mut nums);

    // Replace non-zero entries with 0
    for _itr in 0..60 {
        // Find a random position on the board which is non-zero
        let mut rng = rand::thread_rng();
        let mut r = rng.gen_range(0..dim);
        let mut c = rng.gen_range(0..dim);

        let mut search_c = 0;

        while game.get_value(r, c) == 0 {
            r = rng.gen_range(0..dim);
            c = rng.gen_range(0..dim);
            search_c += 1;

            assert!((search_c < 70), "Not able to find non-zero values!");
        }

        let num_bkup = game.get_value(r, c);
        game.set_value(r, c, 0);

        if !unique_soln(&game, r, c) {
            game.set_value(r, c, num_bkup);
        }
    }

    game
}

#[cfg(test)]
mod tests;
