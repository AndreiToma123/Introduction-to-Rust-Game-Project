use std::io::ErrorKind::ArgumentListTooLong;

const BASE_HEALTH: f32 = 20.0;
const BASE_ARMOR: f32 = 5.0;
const BASE_DAMAGE: f32 = 5.0;
const BASE_COINS: f32 = 1.0;

#[derive(Clone)]
pub struct Enemy {
    pub health: f32,
    pub armor: f32,
    pub damage: f32,
    pub coins: i32,
    pub is_elite: bool,
}

impl Enemy {
    pub fn new(
        health: f32, 
        armor: f32, 
        damage: f32, 
        coins: i32, 
        is_elite: bool,
    ) -> Self {
        Self {
            health,
            armor,
            damage,
            coins,
            is_elite,
        }
    }

    pub fn level_multiplier(player_level: i32, is_elite: bool) -> Self {
        let elite_multiplier: f32 = if is_elite { 1.8 } else { 1.0 };
        let level_multiplier = (player_level - 1) as f32;

        let health = (BASE_HEALTH + level_multiplier * 4.0) * elite_multiplier;
        let armor = (BASE_ARMOR + level_multiplier * 1.5) * elite_multiplier;
        let damage = (BASE_DAMAGE + level_multiplier * 2.0) * elite_multiplier;
        let coins = ((BASE_COINS as f32 + level_multiplier * 2.0) * elite_multiplier) as i32;

        Self {
            health,
            armor,
            damage,
            coins,
            is_elite,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0.0
    }

    pub fn take_damage(&mut self, damage_taken: f32, is_blocked: bool) {
        let blocked: f32 = if is_blocked { 1.4 } else { 1.0 };
        let final_damage_taken = (damage_taken - self.armor * blocked).max(0.0);
        self.health -= final_damage_taken;
    }

    pub fn get_coins(&self) -> i32 {
        self.coins
    }
}
