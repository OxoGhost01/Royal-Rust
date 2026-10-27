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

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // {:<12} = left-aligned on 12 chars, {:>3} = right-aligned on 3 chars.
        write!(
            f,
            "{:<12} ATK {:>3} | DEF {:>3} | PV {:>4}",
            self.name, self.attack, self.defense, self.hp
        )
    }
}
