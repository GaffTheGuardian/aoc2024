use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    LeftToRight,                   // →
    RightToLeft,                   // ←
    TopToBottom,                   // ↓
    BottomToTop,                   // ↑
    DiagonalTopLeftToBottomRight,  // ↘
    DiagonalBottomRightToTopLeft,  // ↖
    DiagonalTopRightToBottomLeft,  // ↙
    DiagonalBottomLeftToTopRight,  // ↗
}

// class grid 
struct Grid {
    rows: Vec<Vec<char>>,
    num_rows: usize,
    num_cols: usize,
}

impl Grid {
    /// Initialize a new Grid instance (still learning how this works lol) 
    fn new(rows: Vec<Vec<char>>) -> Self {
        let num_rows = rows.len();
        let num_cols = if num_rows > 0 { rows[0].len() } else { 0 };
        Grid {
            rows,
            num_rows,
            num_cols,
        }
    }

    /// Helper function to get the row and column deltas based on direction.
    fn get_direction_deltas(&self, direction: Direction) -> (isize, isize) {
        match direction {
            Direction::LeftToRight => (0, 1),
            Direction::RightToLeft => (0, -1),
            Direction::TopToBottom => (1, 0),
            Direction::BottomToTop => (-1, 0),
            Direction::DiagonalTopLeftToBottomRight => (1, 1),
            Direction::DiagonalBottomRightToTopLeft => (-1, -1),
            Direction::DiagonalTopRightToBottomLeft => (1, -1),
            Direction::DiagonalBottomLeftToTopRight => (-1, 1),
        }
    }
    /// part 1
    /// Counts all occurrences of the word
    fn count_word_occurrences(&self, word: &str) -> usize {
        let mut count = 0;
        let directions = [
            Direction::LeftToRight,
            Direction::RightToLeft,
            Direction::TopToBottom,
            Direction::BottomToTop,
            Direction::DiagonalTopLeftToBottomRight,
            Direction::DiagonalBottomRightToTopLeft,
            Direction::DiagonalTopRightToBottomLeft,
            Direction::DiagonalBottomLeftToTopRight,
        ];

        for row in 0..self.num_rows {
            for col in 0..self.num_cols {
                for &dir in &directions {
                    if self.search_from_position(row, col, dir, word) {
                        count += 1;
                    }
                }
            }
        }

        count
    }

    /// Searches for a word starting from a specific position in a given direction.
    fn search_from_position(&self, start_row: usize, start_col: usize, direction: Direction, word: &str) -> bool {
        let chars: Vec<char> = word.chars().collect();
        let (delta_row, delta_col) = self.get_direction_deltas(direction);
        let mut current_row = start_row as isize;
        let mut current_col = start_col as isize;

        for &c in &chars {
            // Check boundaries.
            if current_row < 0 || current_row >= self.num_rows as isize || current_col < 0 || current_col >= self.num_cols as isize {
                return false;
            }

            // Compare characters.
            if self.rows[current_row as usize][current_col as usize] != c {
                return false;
            }

            current_row += delta_row;
            current_col += delta_col;
        }

        true
    }
    /// part 2
    /// Counts all X-MAS patterns in the grid.
    fn count_xmas_patterns(&self) -> usize {
        let mut count = 0;

        // Define direction pairs that form an 'X'.
        // Each pair consists of two diagonal directions intersecting at 'A'.
        let direction_pairs = [
            (
                Direction::DiagonalTopLeftToBottomRight,
                Direction::DiagonalTopRightToBottomLeft,
            ),
        ];

        // Iterate through each cell in the grid, exclude the rim (ty lanston).
        for row in 1..self.num_rows-1 {
            for col in 1..self.num_cols-1 {
                // Check if the current cell is 'A'.
                if self.rows[row][col] != 'A' {
                    continue;
                }

                for &(dir1, dir2) in &direction_pairs {
                    // Check "MAS" or "SAM" in direction1.
                    let has_dir1_mas = self.is_part_of_mas(row, col, dir1, true)
                        || self.is_part_of_mas(row, col, dir1, false);
                    // Check "MAS" or "SAM" in direction2.
                    let has_dir2_mas = self.is_part_of_mas(row, col, dir2, true)
                        || self.is_part_of_mas(row, col, dir2, false);

                    // If both directions have at least one valid pattern, count as X-MAS.
                    if has_dir1_mas && has_dir2_mas {
                        count += 1;
                    }
                }
            }
        }

        count
    }

    /// Checks if the pattern "MAS" or "SAM" exists starting from the 'A' in the given direction
    /// - `forwards`: If true, checks for "MAS"; if false, checks for "SAM".
    fn is_part_of_mas(
        &self,
        a_row: usize,
        a_col: usize,
        direction: Direction,
        forwards: bool,
    ) -> bool {
        let pattern = if forwards { "MAS" } else { "SAM" };
        let chars: Vec<char> = pattern.chars().collect();

        // Get direction deltas.
        let (delta_row, delta_col) = self.get_direction_deltas(direction);

        // Calculate positions for M and S relative to A
        let m_row = a_row as isize - delta_row;
        let m_col = a_col as isize - delta_col;
        let s_row = a_row as isize + delta_row;
        let s_col = a_col as isize + delta_col;

        // Check boundaries
        if m_row < 0
            || m_row >= self.num_rows as isize
            || m_col < 0
            || m_col >= self.num_cols as isize
            || s_row < 0
            || s_row >= self.num_rows as isize
            || s_col < 0
            || s_col >= self.num_cols as isize
        {
            return false;
        }

        let m_char = self.rows[m_row as usize][m_col as usize];
        let s_char = self.rows[s_row as usize][s_col as usize];

        // Compare with the pattern.
        m_char == chars[0] && self.rows[a_row][a_col] == chars[1] && s_char == chars[2]
    }
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut grid_rows = Vec::new();
    for line_result in reader.lines() {
        let line = line_result?;
        let chars: Vec<char> = line.trim().chars().collect();
        grid_rows.push(chars);
    }

    // Initialize the grid (class intialization basically)
    let grid = Grid::new(grid_rows);

    let word_part1 = "XMAS";
    let count_part1 = grid.count_word_occurrences(word_part1);
    println!(
        "Part 1: The word '{}' appears {} times in the grid.",
        word_part1, count_part1
    );

    let count_part2 = grid.count_xmas_patterns();
    println!(
        "Part 2: The X-MAS pattern appears {} times in the grid.",
        count_part2
    );

    Ok(())
}

