use rand::seq::IndexedRandom;

#[derive(Clone, PartialEq)]

pub enum ItemCategory {
    Armor,
    InHand,
    Potion,
}

#[derive(Clone)]
pub struct DefaultGear {
    name: String,
    stat: i32,
    pub category: ItemCategory,
}

impl DefaultGear {
    pub fn new(name: String, stat: i32, category: ItemCategory) -> Self {
        Self {
            name: name.to_string(),
            stat,
            category,
        }
    }

    pub fn get_default_item_name(&self) -> &String {
        &self.name
    }

    pub fn get_default_item_stat(&self) -> &i32 {
        &self.stat
    }

    pub fn base_gear_list() -> Vec<(String, i32, ItemCategory)> {
        vec![
            (String::from("Helmet"), 2, ItemCategory::Armor),
            (String::from("Chestplate"), 2, ItemCategory::Armor),
            (String::from("Boots"), 1, ItemCategory::Armor),
            (String::from("Shield"), 2, ItemCategory::InHand),
            (String::from("Sword"), 2, ItemCategory::InHand),
            (String::from("Dagger"), 1, ItemCategory::InHand),
            (
                String::from("Attack boost potion (+2)"),
                2,
                ItemCategory::Potion,
            ),
            (
                String::from("Armor boost potion (+2)"),
                2,
                ItemCategory::Potion,
            ),
            (String::from("Healing potion (+2)"), 2, ItemCategory::Potion),
        ]
    }

    pub fn set_default_gear() -> (Vec<DefaultGear>, Vec<DefaultGear>, Vec<DefaultGear>) {
        let armor = vec![
            Self::new(String::from("Helmet"), 2, ItemCategory::Armor),
            Self::new(String::from("Chestplate"), 2, ItemCategory::Armor),
            Self::new(String::from("Boots"), 1, ItemCategory::Armor),
        ];

        let in_hand = vec![
            Self::new(String::from("Shield"), 2, ItemCategory::InHand),
            Self::new(String::from("Sword"), 2, ItemCategory::InHand),
            Self::new(String::from("Dagger"), 1, ItemCategory::InHand),
        ];

        let potions = vec![
            Self::new(
                String::from("Attack boost potion (+2)"),
                2,
                ItemCategory::Potion,
            ),
            Self::new(
                String::from("Armor boost potion (+2)"),
                2,
                ItemCategory::Potion,
            ),
            Self::new(String::from("Healing potion (+2)"), 2, ItemCategory::Potion),
        ];

        (armor, in_hand, potions)
    }

    pub fn set_random_starting_gear() -> Vec<DefaultGear> {
        let (armor, in_hand, potions) = Self::set_default_gear();
        let mut random_number = rand::rng();
        let mut final_starting_gear = Vec::new();

        for i in [armor, in_hand, potions] {
            let picked: Vec<DefaultGear> = i.sample(&mut random_number, 2).cloned().collect();
            final_starting_gear.extend(picked);
        }
        final_starting_gear
    }
}
