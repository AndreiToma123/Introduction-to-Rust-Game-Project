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
    stat: f32,
    pub category: ItemCategory,
}

impl DefaultGear {
    pub fn new(name: String, stat: f32, category: ItemCategory) -> Self {
        Self {
            name: name.to_string(),
            stat,
            category,
        }
    }

    pub fn get_default_item_name(&self) -> &String {
        &self.name
    }

    pub fn get_default_item_stat(&self) -> &f32 {
        &self.stat
    }

    pub fn base_gear_list() -> Vec<(String, f32, ItemCategory)> {
        vec![
            (String::from("Helmet"), 1.5, ItemCategory::Armor),
            (String::from("Chestplate"), 2.0, ItemCategory::Armor),
            (String::from("Boots"), 1.0, ItemCategory::Armor),
            (String::from("Spear"), 2.0, ItemCategory::InHand),
            (String::from("Sword"), 1.5, ItemCategory::InHand),
            (String::from("Dagger"), 1.0, ItemCategory::InHand),
            (
                String::from("Attack boost potion"),
                5.0,
                ItemCategory::Potion,
            ),
            (
                String::from("Armor boost potion"),
                4.0,
                ItemCategory::Potion,
            ),
            (String::from("Healing potion"), 10.0, ItemCategory::Potion),
        ]
    }

    pub fn set_default_gear() -> (Vec<DefaultGear>, Vec<DefaultGear>, Vec<DefaultGear>) {
        let armor = vec![
            Self::new(String::from("Helmet"), 1.5, ItemCategory::Armor),
            Self::new(String::from("Chestplate"), 2.0, ItemCategory::Armor),
            Self::new(String::from("Boots"), 1.0, ItemCategory::Armor),
        ];

        let in_hand = vec![
            Self::new(String::from("Spear"), 2.0, ItemCategory::InHand),
            Self::new(String::from("Sword"), 1.5, ItemCategory::InHand),
            Self::new(String::from("Dagger"), 1.0, ItemCategory::InHand),
        ];

        let potions = vec![
            Self::new(
                String::from("Attack boost potion"),
                5.0,
                ItemCategory::Potion,
            ),
            Self::new(
                String::from("Armor boost potion"),
                4.0,
                ItemCategory::Potion,
            ),
            Self::new(String::from("Healing potion"), 10.0, ItemCategory::Potion),
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
