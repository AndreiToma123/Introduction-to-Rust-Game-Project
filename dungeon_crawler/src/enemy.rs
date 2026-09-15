pub struct Enemy {
    health: u64,
    armor: u64,
    base_damage: u64,
    loot_coins: u64,
    loot: Vec<String>,
    level: u64,
    is_elite: bool,
    }