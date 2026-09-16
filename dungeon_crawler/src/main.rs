use std::io;
use std::process::exit;
use crate::combat::start_combat;
use crate::items::DefaultGear;
use crate::player::Player;
mod combat;
mod enemy;
mod items;
mod player;
mod shop;

fn main() {
    println!("Welcome to my dungeon crawler game!\nInstructions:...");
    println!("\nPress 1 to start, 0 to exit.");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");
    let user_choice: i32 = match input.trim().parse() {
        Ok(number) => number,
        Err(_) => {
            println!("Please input a number.");
            return;
        }
    };

    match user_choice {
        1 => {
            let mut player = Player::new(50.0, 0.0, 5.0, 0, 1);
            for item in DefaultGear::set_random_starting_gear() {
                player.add_starting_gear(item);
            }
            start_combat(player);
        },
        0 => exit(0),
        _ => {
            println!("Please type 1 to start the game or 0 to exit.");
            return;
        }
    }
}
