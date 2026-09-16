use crate::items::{DefaultGear, ItemCategory};

const BASE_HEALTH: f32 = 50.0;
const BASE_DAMAGE: f32 = 5.0;
pub struct Player {
    health: f32,
    armor: f32,
    damage: f32,
    coins: i32,
    level: i32,

    armor_slots: Vec<DefaultGear>,
    in_hand_slots: Vec<DefaultGear>,
    potion_slots: Vec<DefaultGear>,
}
impl Player {
    pub fn new(
        health: f32,
        armor: f32,
        damage: f32,
        coins: i32,
        level: i32,
        // armor_slots: Vec<DefaultGear>,
        // in_hand_slots: Vec<DefaultGear>,
        // potion_slots: Vec<DefaultGear>,
    ) -> Self {
        Self {
            health,
            armor,
            damage,
            coins,
            level,
            armor_slots: Vec::new(),
            in_hand_slots: Vec::new(),
            potion_slots: Vec::new(),
        }
    }

    pub fn successful_encounter(&mut self, earned_xp: i32, earned_coins: i32) {
        self.level += earned_xp;
        self.coins += earned_coins;

        self.health = BASE_HEALTH + self.level as f32 * 4.0;
        self.armor = self.level as f32 * 1.5;
        self.damage = BASE_DAMAGE + self.level as f32 * 2.0;
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0.0
    }

    pub fn take_damage(&mut self, damage_taken: f32, is_blocked: bool) {
        let blocked: f32 = if is_blocked { 0.4 } else { 1.0 };
        let final_damage_taken = (damage_taken - self.armor * blocked).max(0.0);
        self.health -= final_damage_taken;
    }

    pub fn total_armor(&self) -> f32 {
        self.armor
            + self
                .armor_slots
                .iter()
                .map(|g| *g.get_default_item_stat() as f32)
                .sum::<f32>()
    }

    pub fn total_damage(&self) -> f32 {
        self.damage
            + self
                .in_hand_slots
                .iter()
                .map(|g| *g.get_default_item_stat() as f32)
                .sum::<f32>()
    }

    pub fn equip(&mut self, item: DefaultGear, slot: usize) -> Result<Option<DefaultGear>, String> {
        let slots = match item.category {
            ItemCategory::Armor => &mut self.armor_slots,
            ItemCategory::InHand => &mut self.in_hand_slots,
            ItemCategory::Potion => &mut self.potion_slots,
        };

        if slot < slots.len() {
            Ok(Some(std::mem::replace(&mut slots[slot], item)))
        } else if slots.len() < 2 {
            slots.push(item);
            Ok(None)
        } else {
            Err(String::from("Invalid slot index"))
        }
    }

    pub fn use_potion(&mut self, slot: usize) -> Option<i32> {
        if slot < self.potion_slots.len() {
            let potion = self.potion_slots.remove(slot);
            Some(*potion.get_default_item_stat())
        } else {
            None
        }
    }

    pub fn add_starting_gear(&mut self, item: DefaultGear) {
        let slots = match item.category {
            ItemCategory::Armor => &mut self.armor_slots,
            ItemCategory::InHand => &mut self.in_hand_slots,
            ItemCategory::Potion => &mut self.potion_slots,
        };

        slots.push(item);
    }
}
