use crate::enemy::Enemy;
use crate::items::DefaultGear;
use crate::player::Player;
use rand::RngExt;
use rand::seq::IndexedRandom;
use std::io;

pub fn start_combat(mut player: Player) {
    println!("The game has started!");
    let mut turn_counter = 0;
    let mut encounter_counter = 1;
    let mut elite_encounter = false;
    let mut elites_defeated = 0;
    loop {
        println!("Choose your next action: 1) Fight enemy 2) Search for treasure. 3) Try shopping");
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
        
        turn_counter += 1;

        match user_choice {
            1 => {
                if encounter_counter % 5 == 0 {
                    elite_encounter = true;
                }

                let mut enemy = Enemy::level_multiplier(elites_defeated, elite_encounter);
                println!("You encounter a new enemy.");

                loop {
                    println!(
                        "Your Stats: HP: {}, Armor: {}, Attack: {}",
                        player.health(),
                        player.total_armor(),
                        player.total_damage()
                    );
                    println!(
                        "Enemy Stats: HP: {}, Armor: {}, Attack: {}",
                        enemy.health, enemy.armor, enemy.damage
                    );
                    println!("1) Attack 2) Block 3) Use potion");

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
                            let mut enemy_attack = false;
                            let mut enemy_choice = rand::rng().random_range(0..2);
                            if enemy_choice % 2 == 0 {
                                enemy_attack = true;
                                println!("The enemy will also attack.");
                            } else {
                                println!("The enemy will block.");
                            }

                            enemy.take_damage(player.total_damage(), !enemy_attack);

                            if !enemy.is_alive() {
                                println!("You defeated the enemy!");
                                break;
                            }

                            if enemy_attack {
                                player.take_damage(enemy.damage, false);
                            }
                        }
                        2 => {
                            player.take_damage(enemy.damage, true);
                        }
                        3 => {
                            if player.potion_slots().is_empty() {
                                println!("You have no potions.");
                                continue;
                            }

                            println!("Consume a potion: ");
                            for (i, potion) in player.potion_slots().iter().enumerate() {
                                println!("{}) {}", i + 1, potion.get_default_item_name())
                            }

                            let mut input = String::new();
                            io::stdin()
                                .read_line(&mut input)
                                .expect("Failed to read line.");
                            let potion_choice: usize = match input.trim().parse() {
                                Ok(number) => number,
                                Err(_) => {
                                    println!("Please input a number.");
                                    continue;
                                }
                            };

                            if potion_choice == 0 || potion_choice > player.potion_slots().len() {
                                println!("That's not a valid potion slot");
                                continue;
                            }
                            player.use_potion(potion_choice - 1);
                            player.take_damage(enemy.damage, false);
                        }
                        _ => {
                            println!("Please input a valid option.");
                            return;
                        }
                    };

                    if !player.is_alive() {
                        println!("You have been defeated. Game over.");
                        return;
                    }
                }
                //Level up and loot logic here
                if enemy.is_elite {
                    player.successful_encounter(1, enemy.get_coins());
                    elites_defeated += 1;
                } else {
                    player.successful_encounter(0, enemy.get_coins());
                }
                if elites_defeated == 3 {
                    println!("You have won the game! Congratulations!");
                    break;
                }
                encounter_counter += 1;
                elite_encounter = false;
            }
            2 => {
                let random_treasure_chance = rand::rng().random_range(0..4);
                if random_treasure_chance == 1 {
                    let treasure_value = rand::rng().random_range(0..4);
                    player.add_coins(treasure_value);
                    println!("You found some treasure!");
                    println!("It's {} gold coins!", treasure_value);
                } else {
                    println!("You found nothing this turn... Maybe next time.");
                }
            }
            3 => {
                if turn_counter % 3 == 0 {
                    println!("You can shop!");
                    //rest of shop logic
                } else {
                    println!("The shop is closed... Try again later.");
                    continue;
                }
            }
            _ => {
                println!("Invalid choice. Please try again.");
                return;
            }
        };
    }
}
