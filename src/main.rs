use std::io;

const PLAYER_X: char = 'X';
const PLAYER_O: char = 'O';
type BOARD = [[char;3];3];

fn intialize_board() -> BOARD{
    return [[' ';3];3];
}

fn print_board(board: &BOARD){
    for row in board{
        for cell in row{
            print!("{} ", cell);
        }
        println!("");
    }
}

fn play_game(){
    let mut game_board = intialize_board();
    let mut curr_player = PLAYER_X;
    let mut curr_input = String::from("");

    loop{
        println!("Current Board: ");
        print_board(&game_board);

        curr_player =  if curr_player == PLAYER_X{
            PLAYER_O
        } else {
            PLAYER_X
        };

        println!("Player {} move (row, col): ", curr_player);
        
        io::stdin()
            .read_line(&mut curr_input)
            .expect("Failed to read line.");

        let moves: Vec<&str> = curr_input.split(',').collect();
        let moves: Vec<u8> = moves.iter().filter_map(|x| x.parse().ok()).collect();
        
        game_board[moves[0]][moves[1]] = curr_player;

    }
    
    
}
fn main() {
    println!("Welcome to rustactoe");
    play_game();

    



}
