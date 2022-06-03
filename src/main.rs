use std::io::stdin;
use std::env;

struct Board {
    height: usize,
    width: usize,
    board: Vec<Vec<char>>
}

impl Board {
    fn new(width: usize, height: usize) -> Self {
        let mut board = Vec::with_capacity(height);
        for _ in 0..height {
            board.push(vec!['-'; width]);
        }
        return Board {
            width,
            height,
            board,
        }
    }

    fn print(&self) {
        for r in 0..self.height {
            for c in 0..self.width {
                let value = self.board[r][c];
                match value {
                    'X' => print!("\u{001b}[31m{}\u{001b}[0m ", value), // Red
                    'O' => print!("\u{001b}[32m{}\u{001b}[0m ", value), // Green
                    _ => print!("{} ", value),
                }
            }
            println!();
        }
        for n in 0..self.width {
            print!("{} ", n);
        }
        println!();
    }

    fn get(&self, row: usize, column: usize) -> char {
        return self.board[row][column];
    }

    fn set(&mut self, row: usize, column: usize, value: char) {
        self.board[row][column] = value;
    }
}

struct Game {
    turn: u32,
    board: Board,
    win_length: usize,
}

impl Game {
    fn new(board_width: usize, board_height: usize, win_length: usize) -> Self {
        let board = Board::new(board_width, board_height);
        return Game {
            turn: 1,
            board,
            win_length,
        }
    }

    fn get_move(&self) -> usize {
        loop {
            // Get string input
            let input = get_input("Enter move (column number):");
            // Parse string into number
            let number: usize;
            match input.parse::<usize>() {
                Ok(n) => number = n,
                Err(_error) => continue,
            }
            // Number is valid column on board
            if number < self.board.width {
                return number;
            }
        }
    }

    fn get_player(&self) -> char {
        return if self.turn % 2 == 1 { 'X' } else { 'O' };
    }

    fn make_move(&mut self, column: usize) {
        let mut row = self.board.height - 1;
        while self.board.get(row, column) != '-' {
            if row == 0 {
                // Column is full
                return;
            }
            row -= 1;
        }
        self.board.set(row, column, self.get_player());
    }

    fn check_win(&self) -> bool {
        let players = vec!['X', 'O'];
        // Check rows
        for player in players.iter() {
            for r in 0..self.board.height {
                for c in 0..(self.board.width - self.win_length) {
                    let mut count = 0;
                    for i in c..(c + self.win_length) {
                        if self.board.get(r, i) == *player {
                            count += 1;
                        }
                    }
                    if count == self.win_length {
                        return true;
                    }
                }
            }
        }
        // Check columns
        for player in players.iter() {
            for c in 0..self.board.width {
                for r in 0..(self.board.height - self.win_length) {
                    let mut count = 0;
                    for i in r..(r + self.win_length) {
                        if self.board.get(i, c) == *player {
                            count += 1;
                        }
                    }
                    if count == self.win_length {
                        return true;
                    }
                }
            }
        }
        // Check diagonals
        // Boards smaller than 'win_length' by 'win_length' cannot form winning diagonals
        if self.board.height < self.win_length || self.board.width < self.win_length {
            return false;
        }
        // Lower left to upper right
        for player in players.iter() {
            for c in 0..=(self.board.width - self.win_length) {
                for r in (self.win_length - 1)..self.board.height {
                    let mut count = 0;
                    for n in 0..self.win_length {
                        if self.board.get(r - n, c + n) == *player {
                            count += 1;
                        }
                    }
                    if count == self.win_length {
                        return true;
                    }
                }
            }
        }
        // Upper left to lower right
        for player in players.iter() {
            for c in 0..=(self.board.width - self.win_length) {
                for r in 0..=(self.board.height - self.win_length) {
                    let mut count = 0;
                    for n in 0..self.win_length {
                        if self.board.get(r + n, c + n) == *player {
                            count += 1;
                        }
                    }
                    if count == self.win_length {
                        return true;
                    }
                }
            }
        }

        return false;
    }

    fn is_full(&self) -> bool {
        for r in 0..self.board.height {
            for c in 0..self.board.width {
                if self.board.get(r, c) == '-' {
                    return false;
                }
            }
        }
        return true;
    }
}

fn get_input(prompt: &str) -> String {
    // Read input string 
    let mut input = String::new();
    println!("{}", prompt);
    stdin().read_line(&mut input)
        .expect("Failed to read input :(");
    // Only look at first line (remove "\n")
    match input.lines().next() {
        Some(value) => input = value.to_string(),
        None => println!("Could not read first line"),
    }
    return input;
}

fn select_game() -> Game {
    println!("Welcome to Connect Four And More :)");
    println!("By: Rohan Phanse");
    println!();
    println!("Game types: ");
    println!("  - tiny: 5 x 5 board, win length of 3");
    println!("  - regular: 7 x 6 board, win length of 4");
    println!("  - big: 12 x 9 board, win length of 5");
    println!();
    loop {
        let input = get_input("Select game type (tiny, regular, big): ");
        if input == "tiny"      { return Game::new(5, 5, 3); }
        if input == "regular"   { return Game::new(7, 6, 4); }
        if input == "big"       { return Game::new(12, 9, 5); }

    }
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let mut game = select_game();
    loop {
        // Clear terminal, print board
        print!("\x1B[2J\x1B[1;1H");
        println!("Connect Four And More :)");
        println!("By: Rohan Phanse");
        game.board.print();
        // Check for win
        if game.check_win() {
            game.turn -= 1;
            println!("{} won the game!", game.get_player());
            break;
        }
        // Board is full, no more moves
        if game.is_full() {
            println!("Tie!");
            break;
        }
        // Move
        let column = game.get_move();
        game.make_move(column);
        game.turn += 1;
    }
}