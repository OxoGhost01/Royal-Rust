use std::fmt;

use serde::{Deserialize, Serialize};

// derive ask rust to create us the basic fn we need (print, clone, etc)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Unit {
    pub name: String,
    pub attack: u32,
    pub defense: u32,
    pub hp: u32,
}

impl Unit {
    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }

    // `saturating_sub` = a subtraction that stops at 0 instead of crashing (rust is soooo cool with this one)
    pub fn damage_against(&self, target: &Unit) -> u32 {
        let damage = self.attack.saturating_sub(target.defense);
        if damage < 1 {
            return 1;
        }
        damage
    }

    pub fn take_damage(&mut self, amount: u32) {
        self.hp = self.hp.saturating_sub(amount);
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // {:<23} = left-aligned on 23 chars, {:>3} = right-aligned on 4 chars.
        write!(f, "{:<23} ATK {:>3} | DEF {:>3} | PV {:>4}", self.name, self.attack, self.defense, self.hp)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreEntry {
    pub player_name: String,
    pub waves: u32,
}
