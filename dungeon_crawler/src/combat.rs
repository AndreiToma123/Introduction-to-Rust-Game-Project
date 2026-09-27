use crate::enemy::Enemy;
use crate::items::{DefaultGear, ItemCategory};
use crate::player::Player;
use crate::shop::Shop;
use rand::RngExt;
use rand::seq::IndexedRandom;
use std::io;

fn print_gear(label: &str, items: &Vec<DefaultGear>) {
    println!("\n{}:", label);
    if items.is_empty() {
        println!("empty");
    } else {
        for item in items {
            println!(
                "{} (+{})",
                item.get_default_item_name(),
                item.get_default_item_stat()
            );
        }
    }
}

pub fn start_combat(mut player: Player) {
    println!("The game has started!");
    let mut turn_counter = 0;
    let mut encounter_counter = 1;
    let mut elite_encounter = false;
    let mut elites_defeated = 0;
    loop {
        println!(
            "\nChoose your next action:\n| 1) Fight enemy | 2) Search for treasure | 3) Try shopping | 4) Check your gear |"
        );
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        let user_choice: i32 = match input.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Please input a number.");
                continue;;
            }
        };

        turn_counter += 1;

        match user_choice {
            1 => {
                //Encounter logic
                if encounter_counter % 5 == 0 {
                    elite_encounter = true;
                    println!(
                        "\nYou encounter an *elite* enemy! This one seems stronger than the rest..."
                    );
                } else {
                    println!("\nYou encounter a new enemy.");
                }

                let mut enemy = Enemy::level_multiplier(elites_defeated, elite_encounter);
                let mut attack_boost: f32 = 0.0;
                loop {
                    let mut enemy_attack = false;
                    let mut enemy_choice = rand::rng().random_range(0..2);
                    if enemy_choice % 2 == 0 {
                        enemy_attack = true;
                        println!(
                            "\nThe enemy will *attack*. They will deal {} damage.\n",
                            enemy.damage
                        );
                    } else {
                        println!("\nThe enemy will *block* this turn.\n");
                    }
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
                    println!("\n| 1) Attack | 2) Block | 3) Use potion |");

                    let mut input = String::new();
                    io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read line.");

                    let user_choice: i32 = match input.trim().parse() {
                        Ok(number) => number,
                        Err(_) => {
                            println!("Please input a number.");
                            continue;
                        }
                    };

                    match user_choice {
                        1 => {
                            //Attack logic
                            let damage_dealt = player.total_damage() + attack_boost;

                            if attack_boost > 0.0 {
                                println!(
                                    "The potion boosted your attack by {} damage.",
                                    attack_boost
                                );
                            }

                            attack_boost = 0.0;

                            enemy.take_damage(damage_dealt, !enemy_attack);

                            if !enemy.is_alive() {
                                println!("\nYou defeated the enemy!\n");
                                println!("You earned {} gold coins.", enemy.get_coins());
                                if enemy.is_elite {
                                    println!("You leveled up.\nYour stats increased.\nYou unlocked new items in the shop.\n");
                                }
                                break;
                            }

                            if enemy_attack {
                                player.take_damage(enemy.damage, false);
                            }
                        }
                        2 => {
                            //Block logic
                            if enemy_attack {
                                player.take_damage(enemy.damage, true);
                            } else {
                                println!(
                                    "\nYou both choose to block. You are just staring at each other...\n"
                                );
                            }
                        }
                        3 => {
                            // Potion logic
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
                            let potion_name = player.potion_slots()[potion_choice - 1]
                                .get_default_item_name()
                                .clone();
                            let effect_value =
                                player.use_potion(potion_choice - 1).unwrap_or(0.0) as f32;

                            let mut armor_boost_this_turn = 0.0;
                            if potion_name.contains("Healing") {
                                player.heal(effect_value);
                                println!("You have restored {} HP.", effect_value);
                            } else if potion_name.contains("Attack") {
                                attack_boost += effect_value;
                                println!(
                                    "Your next attack will be boosted by {} damage.",
                                    attack_boost
                                );
                            } else if potion_name.contains("Armor") {
                                armor_boost_this_turn = effect_value;
                                println!(
                                    "Your armor will be boosted by {} for a turn.",
                                    armor_boost_this_turn
                                );
                            }
                            if enemy_attack {
                                let incoming_damage = (enemy.damage - armor_boost_this_turn).max(0.0);
                                player.take_damage(incoming_damage, false);
                            }
                        }
                        _ => {
                            println!("Please input a valid option.");
                            continue;
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
                    println!("You won the game! Congratulations!");
                    break;
                }
                encounter_counter += 1;
                elite_encounter = false;
            }
            2 => {
                //Treasure search logic
                let random_treasure_chance = rand::rng().random_range(0..4);
                if random_treasure_chance == 1 {
                    let treasure_value = rand::rng().random_range(0..5);
                    player.add_coins(treasure_value);
                    println!("\n*You found some treasure!*");
                    println!("*It's {} gold coins!*\n", treasure_value);
                } else {
                    println!("\nYou found nothing this turn... Maybe next time.\n");
                }
            }
            3 => {
                //Shop logic
                if turn_counter % 3 != 0 {
                    println!("The shop is closed... Try again later.");
                    continue;
                }

                let mut shop = Shop::create_stock(elites_defeated);

                loop {
                    if shop.stock.is_empty() {
                        println!(
                            "The shop is out of stock. Return after you defeat the next Elite!"
                        );
                        break;
                    }
                    println!("You have {} gold coins.", player.coins());
                    println!("Shop inventory");
                    for (i, listing) in shop.stock.iter().enumerate() {
                        println!(
                            "{}) {} [{}] (+{}) - {} coins",
                            i + 1,
                            listing.gear.get_default_item_name(),
                            listing.rarity.label(),
                            listing.gear.get_default_item_stat(),
                            listing.price,
                        );
                    }

                    println!("Enter an item number to buy, or 0 to leave the shop.");

                    let mut shop_input = String::new();
                    io::stdin()
                        .read_line(&mut shop_input)
                        .expect("Failed to read line.");

                    let shop_choice: usize = match shop_input.trim().parse() {
                        Ok(number) => number,
                        Err(_) => {
                            println!("Please input a number.");
                            continue;
                        }
                    };

                    if shop_choice == 0 {
                        break;
                    }

                    match shop.buy(shop_choice - 1, &mut player) {
                        Ok(gear) => {
                            println!("You bought {}", gear.get_default_item_name());

                            let existing: Vec<&DefaultGear> = match gear.category {
                                ItemCategory::Armor => player.armor_slots().iter().collect(),
                                ItemCategory::InHand => player.in_hand_slots().iter().collect(),
                                ItemCategory::Potion => player.potion_slots().iter().collect(),
                            };

                            println!("Choose a slot to equip it in:");
                            for slot in 0..2 {
                                match existing.get(slot) {
                                    Some(item) => println!(
                                        "{}) {} (currently equipped)",
                                        slot + 1,
                                        item.get_default_item_name()
                                    ),
                                    None => println!("{}) (empty)", slot + 1),
                                }
                            }

                            let mut slot_input = String::new();
                            io::stdin()
                                .read_line(&mut slot_input)
                                .expect("Failed to read line.");

                            let slot_choice: usize = match slot_input.trim().parse::<usize>() {
                                Ok(number) if number == 1 || number == 2 => number - 1,
                                _ => {
                                    println!("Invalid slot.");
                                    continue;
                                }
                            };

                            match player.equip(gear, slot_choice) {
                                Ok(Some(old_item)) => {
                                    println!(
                                        "You swapped out {}.",
                                        old_item.get_default_item_name()
                                    );
                                }
                                Ok(None) => {
                                    println!("Item equipped.");
                                }
                                Err(msg) => {
                                    println!("{}", msg);
                                }
                            }
                        }

                        Err(msg) => {
                            println!("{}", msg);
                        }
                    }
                }
            }

            4 => {
                //Check gear logic
                println!("Your equipment:");
                print_gear("Armor", player.armor_slots());
                print_gear("In hand", player.in_hand_slots());
                print_gear("Potions", player.potion_slots());
            }
            _ => {
                println!("Invalid choice. Please try again.");
                continue;
            }
        };
    }
}
