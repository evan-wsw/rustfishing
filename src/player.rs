use std::collections::HashMap;

use crate::fish::*;
pub struct Player {
    pub lure: Lure,
    pub lures: Vec<Lure>,
    pub rod_tier: i32,
    pub catch_chance: f64,
    pub bait_capacity: i32,
    pub bait: HashMap<Bait, i32>,
    pub current_bait:Bait,
    pub money: i32,
    pub inventory: Vec<Fish>
}
#[derive (PartialEq)]
pub enum Lure {
    Basic, // no special effects
    Fly, // picks smallest fish possible out of the 3 rolls
    Large, // picks largest fish possible out of the 3 rolls
    Efficient, // 25% chance to not subtract bait when fishing
    Sparkling, // picks highest tier fish out of the 3 rolls, 25% chance to consume extra bait
    //test
}
#[derive (PartialEq, Eq, Hash, Debug, Clone)]
pub enum Bait {
    Worm,
    Leech,
    Minnow,
}
impl Player {
    pub fn new() -> Self {
        Self {
            lure: Lure::Basic,
            lures: vec![Lure::Basic],
            rod_tier: 1,
            catch_chance: 0.5,
            bait_capacity: 10,
            bait: HashMap::new(),
            current_bait: Bait::Worm,
            money: 100,
            inventory: Vec::new(),
        }
    }
}
