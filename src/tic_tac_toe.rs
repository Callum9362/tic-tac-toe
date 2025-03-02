pub struct TicTacToe {
    pub board: Vec<Vec<char>>,
}

impl TicTacToe {
    pub fn new() -> Self {
        Self {
            board: vec![vec![' '; 3], vec![' '; 3], vec![' '; 3]],
        }
    }

    pub fn display_board(&self) {
        for row in &self.board {
            println!("{}", row.iter().map(
                |c| c.to_string()).collect::<Vec<_>>().join(" | ")
            );
        }
        println!("---------");
    }

    pub fn make_move(&mut self, row: usize, col: usize, player: char) -> bool {
        if row < 3 && col < 3 && self.board[row][col] == ' ' {
            self.board[row][col] = player;
            true
        } else {
            false
        }
    }

    pub fn check_winner(&self, player: char) -> bool {
        for i in 0..3 {
            if self.board[i].iter().all(|&c| c == player)
                || (0..3).all(|j| self.board[j][i] == player) {
                return true;
            }
        }
        if (0..3).all(|i| self.board[i][i] == player)
            || (0..3).all(|i| self.board[i][2 - i] == player) {
            return true;
        }
        false
    }

    pub fn is_tie(&self) -> bool {
        self.board.iter().all(|row| row.iter().all(|&c| c != ' '))
            && !self.check_winner('X') && !self.check_winner('O')
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_board() {
        let game = TicTacToe::new();
        assert_eq!(game.board, vec![vec![' '; 3], vec![' '; 3], vec![' '; 3]]);
    }

    #[test]
    fn test_valid_move() {
        let mut game = TicTacToe::new();
        assert!(game.make_move(0, 0, 'X'));
        assert_eq!(game.board[0][0], 'X');
    }

    #[test]
    fn test_invalid_move_on_occupied_cell() {
        let mut game = TicTacToe::new();
        game.make_move(0, 0, 'X');
        assert!(!game.make_move(0, 0, 'O')); // Cell already taken
    }

    #[test]
    fn test_winner_horizontal() {
        let mut game = TicTacToe::new();
        game.make_move(0, 0, 'X');
        game.make_move(0, 1, 'X');
        game.make_move(0, 2, 'X');
        assert!(game.check_winner('X'));
    }

    #[test]
    fn test_tie_condition() {
        let game = TicTacToe {
            board: vec![
                vec!['X', 'O', 'X'],
                vec!['X', 'X', 'O'],
                vec!['O', 'X', 'O'],
            ],
        };
        assert!(game.is_tie());
    }
}