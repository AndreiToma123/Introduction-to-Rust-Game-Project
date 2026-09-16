use crate::items::DefaultGear;
use crate::player::Player;
mod combat;
mod enemy;
mod items;
mod player;
mod shop;

fn main() {
    let mut player = Player::new(50.0, 0.0, 5.0, 0, 1);
    for item in DefaultGear::set_random_starting_gear() {
        player.add_starting_gear(item);
    }
}
