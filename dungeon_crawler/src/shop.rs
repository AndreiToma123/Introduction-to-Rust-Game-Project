use crate::items::{DefaultGear, ItemCategory};
use crate::player::Player;

#[derive(Clone, PartialEq)]
pub enum Rarity {
    Uncommon,
    Rare,
    Legendary,
}

pub struct ShopStock {
    pub gear: DefaultGear,
    pub rarity: Rarity,
    pub price: i32,
}

pub struct Shop {
    pub stock: Vec<ShopStock>,
}

impl Shop {
    pub fn create_stock(elites_defeated: i32) -> Self {
        let rarity = rarity_setter(elites_defeated);
        let multiplier = status_multiplier(&rarity);
        let price = price_tier(&rarity);

        let mut stock = Vec::new();

        for (name, base_stat, category) in DefaultGear::base_gear_list() {
            let gear = DefaultGear::new(name, base_stat * multiplier, category);
            stock.push(ShopStock {
                gear,
                rarity: rarity.clone(),
                price
            });
        }

        Self { stock }
    }

    pub fn buy(&mut self, index: usize, player: &mut Player) -> Result<DefaultGear, String> {
        if index >= self.stock.len() {
            return Err(String::from("You don't have enough coins."));
        }

        let listing = self.stock.remove(index);
        Ok(listing.gear)
    }
}

    impl Rarity {
        pub fn label(&self) -> &str {
            match self {
                Rarity::Uncommon => "Uncommon",
                Rarity::Rare => "Rare",
                Rarity::Legendary => "Legendary",
            }
        }
    }

    fn rarity_setter(elites_defeated: i32) -> Rarity {
        match elites_defeated {
            0 => Rarity::Uncommon,
            1 => Rarity::Rare,
            _ => Rarity::Legendary,
        }
    }

    fn status_multiplier(rarity: &Rarity) -> i32 {
        match rarity {
            Rarity::Uncommon => 2,
            Rarity::Rare => 3,
            Rarity::Legendary => 4,
        }
    }

    fn price_tier(rarity: &Rarity) -> i32 {
        match rarity {
            Rarity::Uncommon => 10,
            Rarity::Rare => 20,
            Rarity::Legendary => 30,
        }
    }



