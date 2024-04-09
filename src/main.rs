use std::fmt;
use std::io;
use std::io::Write;

#[derive(Clone, Copy, PartialEq)]
enum Player {
    X,
    O,
}

impl fmt::Debug for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Player::X => write!(f, "X"),
            Player::O => write!(f, "O"),
        }
    }
}

struct Game {
    board: [[Option<Player>; 3]; 3],
    current_player: Player,
}

impl Game {
    fn new() -> Self {
        Game {
            board: [[None; 3]; 3],
            current_player: Player::X,
        }
    }

    fn print_board(&self) {
        for row in &self.board {
            println!(
                "{} | {} | {}",
                row[0].map_or(" ".to_string(), |p| format!("{:?}", p)),
                row[1].map_or(" ".to_string(), |p| format!("{:?}", p)),
                row[2].map_or(" ".to_string(), |p| format!("{:?}", p))
            );
            println!("---------");
        }
    }

    fn make_move(&mut self, row: usize, col: usize) -> Result<(), &'static str> {
        if row >= 3 || col >= 3 {
            return Err("\nInvalid move. Row and column must be between 0 and 2.\n");
        }
        if self.board[row][col].is_some() {
            return Err("Invalid move. Cell already occupied.");
        }
        self.board[row][col] = Some(self.current_player);
        self.current_player = match self.current_player {
            Player::X => Player::O,
            Player::O => Player::X,
        };
        Ok(())
    }

    fn check_winner(&self) -> Option<Player> {
        // Check rows
        for row in 0..3 {
            if self.board[row][0] == self.board[row][1] && self.board[row][0] == self.board[row][2]
            {
                return self.board[row][0];
            }
        }
        // Check columns
        for col in 0..3 {
            if self.board[0][col] == self.board[1][col] && self.board[0][col] == self.board[2][col]
            {
                return self.board[0][col];
            }
        }
        // Check diagonals
        if self.board[0][0] == self.board[1][1] && self.board[0][0] == self.board[2][2] {
            return self.board[0][0];
        }
        if self.board[0][2] == self.board[1][1] && self.board[0][2] == self.board[2][0] {
            return self.board[0][2];
        }
        None
    }

    fn check_draw(&self) -> bool {
        self.board
            .iter()
            .all(|row| row.iter().all(|&cell| cell.is_some()))
    }
}

fn main() {
    let mut game = Game::new();
    loop {
        println!("Player {:?}'s turn", game.current_player);
        game.print_board();
        println!("Enter row and column (0-2) separated by space:");

        let mut input = String::new();
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let mut coords = input.trim().split_whitespace();

        let row: usize = match coords.next() {
            Some(val) => val.parse().unwrap_or(usize::MAX),
            None => usize::MAX,
        };
        let col: usize = match coords.next() {
            Some(val) => val.parse().unwrap_or(usize::MAX),
            None => usize::MAX,
        };

        if row == usize::MAX || col == usize::MAX {
            println!("Invalid input. Please enter row and column numbers.");
            continue;
        }

        match game.make_move(row, col) {
            Ok(_) => {
                if let Some(winner) = game.check_winner() {
                    game.print_board();
                    println!("Player {:?} wins!", winner);
                    break;
                } else if game.check_draw() {
                    game.print_board();
                    println!("It's a draw!");
                    break;
                }
            }
            Err(err) => println!("{}", err),
        }
    }
}
