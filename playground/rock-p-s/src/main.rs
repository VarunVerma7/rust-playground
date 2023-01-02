use std::io;


enum GameOption {
    Rock,
    Paper,
    Scissors,
}

fn main() {
    let mut user_1_choice = String::from("");
    let mut user_2_choice = String::from("");

    let user_1_int: usize;
    let user_2_int: usize;


    println!("User 1, Rock, Paper, or Scissors?");
    io::stdin().read_line(&mut user_1_choice).expect("Failed to read line");

    user_1_int = user_1_choice.trim().parse().unwrap();

    println!("User 2, Rock, Paper, or Scissors?");
    io::stdin().read_line(&mut user_2_choice).expect("Failed to read line");
    user_2_int = user_2_choice.trim().parse().unwrap();


    println!("User 1 entered {user_1_choice} and user 2 entered {user_2_choice}");

    let user_1_enum = map_input_to_enum(user_1_int);

    let user_2_enum = map_input_to_enum(user_2_int);

}


fn map_input_to_enum(selection: usize) -> GameOption {
    match selection {
        1 => GameOption::Rock,
        2 => GameOption::Paper,
        3 => GameOption::Scissors,
        _ => panic!("UHOH!")
    }
}

fn user_choices_to_winner(user_1: GameOption, user_2: GameOption) -> usize {
    if (user_1 == user_2) {
        0
    } else if 
}