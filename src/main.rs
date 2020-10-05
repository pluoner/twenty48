mod game_logic;
#[cfg(test)]
mod tests;
use std::io;

fn main() {

    let board_size = 4;
    let mut new_game = game_logic::Game::new_game(board_size);


    loop {
        let mut input = String::new();
        if new_game.game_over() {
            println!("Game over, thank you for playing! You scored a whooping {} points. Well done!", new_game.score)
        }
        game_logic::draw_game(&new_game);
        println!("Input direction!");
        io::stdin().read_line(&mut input).expect("Could not read line...");

        match input.trim() {
            "r" => {new_game = game_logic::Game::validate_and_execute_next_state(new_game, &game_logic::Direction::Right)},
            "l" => {new_game = game_logic::Game::validate_and_execute_next_state(new_game, &game_logic::Direction::Left)},
            "u" => {new_game = game_logic::Game::validate_and_execute_next_state(new_game, &game_logic::Direction::Up)},
            "d" => {new_game = game_logic::Game::validate_and_execute_next_state(new_game, &game_logic::Direction::Down)},
            _ => println!("Not a valid input, enter 'l' for left, 'r' for right, 'u' for up or 'd' for down"),
        };
    }
}

