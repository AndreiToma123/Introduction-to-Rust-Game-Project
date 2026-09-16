use rand::seq::IndexedRandom;

#[derive(Clone)]
pub struct DefaultGear {
    name: String,
    stat: i32,
}

// pub struct Shop_Gear {
//     name: String,
//     price: u64,
//     stats: Vec<String>,
//     }

impl DefaultGear {
    pub fn new(name: String, stat: i32) -> Self {
        Self {
            name: name.to_string(),
            stat,
        }
    }

    pub fn get_default_item_name(&self) -> &String {
        &self.name
    }

    pub fn get_default_item_stat(&self) -> &i32 {
        &self.stat
    }

    pub fn set_default_gear() -> (Vec<DefaultGear>, Vec<DefaultGear>, Vec<DefaultGear>) {
        let armor = vec![
            Self::new(String::from("Helmet"), 2),
            Self::new(String::from("Chestplate"), 2),
            Self::new(String::from("Boots"), 1),
        ];

        let in_hand = vec![
            Self::new(String::from("Shield"), 2),
            Self::new(String::from("Sword"), 2),
            Self::new(String::from("Dagger"), 1),
        ];

        let potions = vec![
            Self::new(String::from("Attack boost potion (+2)"), 2),
            Self::new(String::from("Armor boost potion (+2)"), 2),
            Self::new(String::from("Healing potion (+2)"), 2),
        ];

        (armor, in_hand, potions)
    }

    pub fn set_random_starting_gear() -> Vec<DefaultGear> {
        let (armor, in_hand, potions) = Self::set_default_gear();
        let mut random_number= rand::rng();
        let mut final_starting_gear = Vec::new();

        for i in [armor, in_hand, potions] {
            let picked: Vec<DefaultGear> = i
                .sample(&mut random_number, 2)
                .cloned()
                .collect();
            final_starting_gear.extend(picked);
        }
        final_starting_gear
    }
}
