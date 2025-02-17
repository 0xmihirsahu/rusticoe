use std::io;
const BOARD_SIZE: usize = 3;
const PLAYER_X: char = 'X';
const PLAYER_O: char = 'O';
type BOARD = [[char; BOARD_SIZE]; BOARD_SIZE];

fn initialize_board() -> BOARD {
    [[' '; BOARD_SIZE]; BOARD_SIZE]
}

fn print_board(board: &BOARD) {
    for row in board {
        println!("{}", row.iter().map(|&c| c.to_string()).collect::<Vec<_>>().join(" | "));
    }
    println!();
}

fn play_game() {
    let mut game_board = initialize_board();
    let mut curr_player = PLAYER_X;

    loop {
        println!("Current Board: ");
        print_board(&game_board);

        println!("Player {} move (row, col): ", curr_player);
        let (row, col) = get_inputs();

        if game_board[row][col] != ' ' {
            println!("Cell already taken, try again!");
            continue;
        }

        game_board[row][col] = curr_player;

        let winner = check_winner(&game_board);
        if winner != ' ' {
            println!("Player {} wins!", winner);
            break;
        }

        if check_draw(&game_board){
            println!("Game Draw :|");
            break;
        }

        // Toggle player
        curr_player = if curr_player == PLAYER_X {
            PLAYER_O
        } else {
            PLAYER_X
        };
    }
}


fn check_draw(board: &BOARD) -> bool {
    for row in board{
        for col in row{
            if *col == ' ' {
                return false;
            }
        }
    }

    true
}
fn get_inputs() -> (usize, usize) {
    let mut curr_input = String::new();

    loop {
        io::stdin().read_line(&mut curr_input).expect("Failed to read line.");

        let moves: Vec<usize> = curr_input
            .trim()
            .split(',')
            .filter_map(|x| x.parse().ok())
            .collect();

        curr_input.clear(); 

        if moves.len() == 2 {
            let (row, col) = (moves[0], moves[1]);
            if row < BOARD_SIZE && col < BOARD_SIZE {
                return (row, col);
            } else {
                println!("Invalid move! Board size is {}", BOARD_SIZE);
            }
        } else {
            println!("Invalid input! Please enter row,col (e.g., 1,2)");
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
    println!("Welcome to Rustactoe!");
    play_game();
}