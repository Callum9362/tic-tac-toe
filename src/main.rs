mod tic_tac_toe;

use tic_tac_toe::TicTacToe;

fn main() {
    let mut game = TicTacToe::new();
    let mut current_player = 'X';

    loop{
        println!("\nCurrent board:");
        game.display_board();

        println!("Player {current_player}, enter your move (row and column):");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read input");
        let input: Vec<usize> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap_or(0))
            .collect();

        if input.len() != 2 || !game.make_move(input[0], input[1], current_player) {
            println!("Invalid move. Try again.");
            continue;
        }

        if game.check_winner(current_player) {
            println!("\nPlayer {} wins!", current_player);
            game.display_board();
            break;
        }

        if game.is_tie() {
            println!("\nIt's a tie!");
            game.display_board();
            break;
        }

        current_player = if current_player == 'X' { 'O' } else { 'X' };

    }
}
