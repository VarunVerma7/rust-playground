use std::env;
use std::io;
use std::io::*;


fn main() {

    // 0 = no
    let mut game_state = [[0u8; 3]; 3];
    // println!("{}", state[0][0]);
    let mut turn = false;

    loop {
        let player = if turn { "Player 1" } else { "Player 2" };

        println!("Hello, lets play tic tac toe! {} your turn: pick row and column", player);

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
            game_state[row_num][col_num] = 2;

        }
        turn = !turn;

        let winner = check_winner(&mut game_state);
        if (winner) {
            println!("We have a winner");
            print_state(&mut game_state);
            break;

        } else {
            print_state(&mut game_state);
        }

        // println!("Game state {}", game_state[row_num][col_num]);
    }


    // println!("{num}");

}

fn check_winner(grid: &mut [[u8; 3]; 3]) -> bool {
    let grid2 = grid.clone();
    for (i, row) in grid.iter_mut().enumerate() {
        // check row
        if (grid2[i][0] == grid2[i][1] &&  grid2[i][1] == grid2[i][2] && grid2[i][0] != 0) {
            return true;
        }

        // check column
        if (grid2[0][i] == grid2[1][i] &&  grid2[1][i] == grid2[2][i] && grid2[2][i] != 0) {
            return true;
        }

        // check diagonals
    }
    false
}

fn print_state(grid: &mut [[u8; 3]; 3]) {
    let grid2 = grid.clone();
    for (i, row) in grid.iter_mut().enumerate() {
        println!("{}|{}|{}", grid2[i][0], grid2[i][1], grid2[i][2]);
        if (i == 2) {
            break;
        }
    }
}


// fn reset_game_state(&mut game_state) {
//     game_state = [[0u8; 3]; 3];    
// }
