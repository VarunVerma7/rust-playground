use std::env;
use std::io;
use std::io::*;


fn main() {

    // 0 = no
    let mut game_state = [[0u8; 3]; 3];
    // println!("{}", state[0][0]);
    let mut turn = false;

    loop {
        println!("Hello, lets play tic tac toe! Player 1 your turn: pick row and column");

        let mut row = String::new();
        io::stdin()
            .read_line(&mut row)
            .expect("Failed to read line");
    
        let mut col = String::new();
            io::stdin()
                .read_line(&mut col)
                .expect("Failed to read line");
    
        // println!("{row} {column}");
        let row_num: usize = row.trim().parse().unwrap();
        let col_num: usize = col.trim().parse().unwrap();
    
        if turn {
            game_state[row_num][col_num] = 1;
        } else {
            game_state[row_num][col_num] = 0;

        }
        turn = !turn;
        println!("Game state {}", game_state[row_num][col_num]);
    }


    // println!("{num}");

}

fn won_game() {

}

// fn reset_game_state(&mut game_state) {
//     game_state = [[0u8; 3]; 3];    
// }
