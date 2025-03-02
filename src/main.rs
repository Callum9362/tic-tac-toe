use std::io;

fn main() {
    let mut board = vec![vec![' '; 3]; 3];

    let mut current_player = 'X';

    loop{
        print_board(&board);

        println!("Player {current_player}, enter your move (row and column):");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let trimmed = input.trim();

        let coords: Vec<usize> = trimmed
            .split_whitespace()
            .filter_map(|x| x.parse().ok())
            .collect();

        if coords.len() != 2 {
            println!("Please enter two numbers separated by a space (e.g., 0 2).");
            continue;
        }

        let (row, col) = (coords[0], coords[1]);

        if row >= 3 || col >= 3 || board[row][col] != ' ' {
            println!("Invalid move. Try again.");
            continue;
        }

        board[row][col] = current_player;

        if check_win(&board, current_player) {
            print_board(&board);
            println!("Player {current_player} wins!");
            break;
        } else if is_tie(&board) {
            print_board(&board);
            println!("It's a tie!");
            break;
        }

        current_player = if current_player == 'X' { 'O' } else { 'X' };
    }
}

fn print_board(board: &Vec<Vec<char>>) {
    println!("Current board:");
    for row in board {
        println!(
            "{}",
            row.iter()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join(" | ")
        )
    }
    println!();
}

fn check_win(board: &Vec<Vec<char>>, player: char) -> bool {
    for i in 0..3 {
        if board[i][0] == player && board[i][1] == player && board[i][2] == player {
            return true;
        }
        if board[0][i] == player && board[1][i] == player && board[2][i] == player {
            return true;
        }
    }

    false
}

fn is_tie(board: &Vec<Vec<char>>) -> bool {
    for row in board {
        if row.contains(&' ') {
            return false;
        }
    }

    true
}
