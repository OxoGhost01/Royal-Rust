use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Effect {
    pub name: String,
    pub damage: u32,
    pub turns: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Unit {
    pub name: String,
    pub attack: u32,
    pub defense: u32,
    pub hp: u32,

    #[serde(rename = "type", default)]
    pub kind: String,
    #[serde(default)]
    pub effect_chance: f64,
    #[serde(default)]
    pub effect_damage: u32,
    #[serde(default)]
    pub effect_duration: u32,
    #[serde(skip)]
    pub effects: Vec<Effect>,
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

    pub fn effect_name(&self) -> Option<&'static str> {
        match self.kind.as_str() {
            "sorcerer" => Some("poison"),
            "tank" => Some("knocked out"),
            _ => None,
        }
    }

    pub fn tick_effects(&mut self, log: &mut Vec<String>) {
        let effects = std::mem::take(&mut self.effects);
        let mut remaining: Vec<Effect> = Vec::new();

        for mut effect in effects {
            self.take_damage(effect.damage);
            log.push(format!("  {} takes {} {} damage (PV: {})", self.name, effect.damage, effect.name, self.hp));

            effect.turns = effect.turns.saturating_sub(1);
            if effect.turns > 0 {
                remaining.push(effect);
            } else {
                log.push(format!("  {}'s {} wore off", self.name, effect.name));
            }
        }
        self.effects = remaining;
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // {:<23} = left-aligned on 23 chars, {:>3} = right-aligned on 4 chars.
        write!(f, "{:<23} ATK {:>3} | DEF {:>3} | PV {:>4}", self.name, self.attack, self.defense, self.hp)?;

        // what this unit can inflict (only if it has an effect)
        if let Some(effect_name) = self.effect_name() {
            let percent = self.effect_chance * 100.0;
            write!(f, " | {percent:.0}% {effect_name} ({} dmg x{})", self.effect_damage, self.effect_duration)?;
        }
        // what this unit suffers right now
        for effect in &self.effects {
            write!(f, " | [{} {} dmg x{}]", effect.name, effect.damage, effect.turns)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreEntry {
    pub player_name: String,
    pub waves: u32,
}
