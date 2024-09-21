use rand::seq::SliceRandom;
use std::collections::HashSet;
use std::io::{self, Write};

fn main() {
    println!("Welcome to Minesweeper!");

    // Get board size
    let (rows, cols) = loop {
        println!("Enter board size (e.g., 9 9 for a 9x9 board):");
        let mut size_input = String::new();
        io::stdin().read_line(&mut size_input).unwrap();
        if let Some((r, c)) = parse_board_size(&size_input) {
            break (r, c);
        } else {
            println!("Invalid input. Please enter two numbers separated by a space.");
        }
    };

    // Get number of mines
    let num_mines = loop {
        println!("Enter number of mines:");
        let mut mines_input = String::new();
        io::stdin().read_line(&mut mines_input).unwrap();
        if let Ok(n) = mines_input.trim().parse::<usize>() {
            if n < rows * cols {
                break n;
            } else {
                println!("Number of mines must be less than total cells.");
            }
        } else {
            println!("Invalid input. Please enter a number.");
        }
    };

    let mut game = Game::new(rows, cols, num_mines);
    game.play();
}

fn parse_board_size(input: &str) -> Option<(usize, usize)> {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.len() != 2 {
        return None;
    }
    let rows = parts[0].parse::<usize>().ok()?;
    let cols = parts[1].parse::<usize>().ok()?;
    Some((rows, cols))
}

struct Game {
    rows: usize,
    cols: usize,
    board: Vec<Vec<Cell>>,
    revealed: HashSet<(usize, usize)>,
}

#[derive(Clone)]
struct Cell {
    is_mine: bool,
    adjacent_mines: u8,
}

impl Game {
    fn new(rows: usize, cols: usize, num_mines: usize) -> Self {
        let mut board = vec![vec![Cell { is_mine: false, adjacent_mines: 0 }; cols]; rows];

        // Place mines
        let mut positions = Vec::new();
        for r in 0..rows {
            for c in 0..cols {
                positions.push((r, c));
            }
        }
        let mut rng = rand::thread_rng();
        positions.shuffle(&mut rng);
        for &(r, c) in positions.iter().take(num_mines) {
            board[r][c].is_mine = true;
        }

        // Calculate adjacent mines
        for r in 0..rows {
            for c in 0..cols {
                if board[r][c].is_mine {
                    continue;
                }
                let mut count = 0;
                for dr in -1i32..=1 {
                    for dc in -1i32..=1 {
                        if dr == 0 && dc == 0 {
                            continue;
                        }
                        let nr = r as i32 + dr;
                        let nc = c as i32 + dc;
                        if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                            if board[nr as usize][nc as usize].is_mine {
                                count += 1;
                            }
                        }
                    }
                }
                board[r][c].adjacent_mines = count;
            }
        }

        Game {
            rows,
            cols,
            board,
            revealed: HashSet::new(),
        }
    }

    fn play(&mut self) {
        loop {
            self.print_board();
            println!("Enter your move (e.g., A1):");
            let mut move_input = String::new();
            io::stdin().read_line(&mut move_input).unwrap();
            if let Some((row, col)) = self.parse_move(&move_input) {
                if !self.reveal_cell(row, col) {
                    self.print_board_reveal();
                    println!(
                        "Game Over! You hit a mine at {}{}.",
                        (col as u8 + b'A') as char,
                        row + 1
                    );
                    break;
                } else if self.check_win() {
                    self.print_board_reveal();
                    println!("Congratulations! You've cleared the minefield!");
                    break;
                }
            } else {
                println!("Invalid move. Please try again.");
            }
        }
    }

    fn parse_move(&self, input: &str) -> Option<(usize, usize)> {
        let input = input.trim().to_uppercase();
        if input.len() < 2 {
            return None;
        }
        let col_char = input.chars().next()?;
        let row_str = &input[1..];
        let col = (col_char as u8).wrapping_sub(b'A') as usize;
        let row = row_str.parse::<usize>().ok()?.wrapping_sub(1);
        if row < self.rows && col < self.cols {
            Some((row, col))
        } else {
            None
        }
    }

    fn reveal_cell(&mut self, row: usize, col: usize) -> bool {
        if self.revealed.contains(&(row, col)) {
            return true;
        }
        self.revealed.insert((row, col));

        if self.board[row][col].is_mine {
            return false;
        }
        if self.board[row][col].adjacent_mines == 0 {
            for dr in -1i32..=1 {
                for dc in -1i32..=1 {
                    let nr = row as i32 + dr;
                    let nc = col as i32 + dc;
                    if nr >= 0 && nr < self.rows as i32 && nc >= 0 && nc < self.cols as i32 {
                        let pos = (nr as usize, nc as usize);
                        if !self.revealed.contains(&pos) {
                            self.reveal_cell(pos.0, pos.1);
                        }
                    }
                }
            }
        }
        true
    }

    fn print_board(&self) {
        // Print column headers
        print!("   ");
        for c in 0..self.cols {
            print!(" {} ", (c as u8 + b'A') as char);
        }
        println!();

        for r in 0..self.rows {
            // Print row number
            print!("{:2} ", r + 1);
            for c in 0..self.cols {
                let ch = if self.revealed.contains(&(r, c)) {
                    if self.board[r][c].is_mine {
                        '*'
                    } else if self.board[r][c].adjacent_mines > 0 {
                        char::from_digit(self.board[r][c].adjacent_mines as u32, 10).unwrap()
                    } else {
                        ' '
                    }
                } else {
                    '.'
                };
                print!("[{}]", ch);
            }
            println!();
        }
    }

    fn print_board_reveal(&self) {
        // Print column headers
        print!("   ");
        for c in 0..self.cols {
            print!(" {} ", (c as u8 + b'A') as char);
        }
        println!();

        for r in 0..self.rows {
            // Print row number
            print!("{:2} ", r + 1);
            for c in 0..self.cols {
                let ch = if self.board[r][c].is_mine {
                    '*'
                } else if self.board[r][c].adjacent_mines > 0 {
                    char::from_digit(self.board[r][c].adjacent_mines as u32, 10).unwrap()
                } else {
                    ' '
                };
                print!("[{}]", ch);
            }
            println!();
        }
    }

    fn check_win(&self) -> bool {
        let total_cells = self.rows * self.cols;
        self.revealed.len() + self.num_mines() == total_cells
    }

    fn num_mines(&self) -> usize {
        self.board
            .iter()
            .flatten()
            .filter(|cell| cell.is_mine)
            .count()
    }
}
