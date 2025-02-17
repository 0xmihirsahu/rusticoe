use std::io;

const PLAYER_X: char = 'X';
const PLAYER_O: char = 'O';
type BOARD = [[char; 3]; 3];

fn intialize_board() -> BOARD {
    return [[' '; 3]; 3];
}

fn print_board(board: &BOARD) {
    for row in board {
        for cell in row {
            print!("{} ", cell);
        }
        println!("");
    }
}

fn play_game() {
    let mut game_board = intialize_board();
    let mut curr_player = PLAYER_X;
    let mut curr_input = String::from("");

    loop {
        println!("Current Board: ");
        print_board(&game_board);

        curr_player = if curr_player == PLAYER_X {
            PLAYER_O
        } else {
            PLAYER_X
        };

        println!("Player {} move (row, col): ", curr_player);

        io::stdin()
            .read_line(&mut curr_input)
            .expect("Failed to read line.");

        let moves: Vec<usize> = curr_input
            .trim()
            .split(',')
            .filter_map(|x| x.parse().ok())
            .collect();

        curr_input.clear();
        game_board[moves[0]][moves[1]] = curr_player;

        match check_winner(&game_board) {
            PLAYER_X => {
                print!("Player {} wins!", PLAYER_X);
                break;
            }

            PLAYER_O => {
                print!("Player {} wins!", PLAYER_O);
                break;
            }

            _ => {}
        }
    }
}

fn check_winner(board: &BOARD) -> char {
    for row in 0..3 {
        if board[row][0] != ' ' && board[row][0] == board[row][1] && board[row][1] == board[row][2]
        {
            return board[row][0];
        }
    }

    for col in 0..3 {
        if board[0][col] != ' ' && board[0][col] == board[1][col] && board[1][col] == board[2][col]
        {
            return board[0][col];
        }
    }

    if board[0][0] != ' ' && board[0][0] == board[1][1] && board[1][1] == board[2][2] {
        return board[0][0];
    }

    if board[0][2] != ' ' && board[0][2] == board[1][1] && board[1][1] == board[2][0] {
        return board[0][2];
    }

    ' '
}
fn main() {
    println!("Welcome to rustactoe");
    play_game();
}
