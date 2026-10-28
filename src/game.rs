use rand::Rng;

use crate::models::{Effect, Unit};

// What happened during one wave.
pub struct FightReport {
    pub victory: bool,    // true = the monster died, false = the whole team died
    pub log: Vec<String>, // one text line per event, to print afterwards
}

pub fn fight_wave(team: &mut Vec<Unit>, monster: &mut Unit, rng: &mut impl Rng) -> FightReport {
    let mut log: Vec<String> = Vec::new();
    let mut round = 1;

    loop {
        log.push(format!("-- round {round} --"));

        monster.tick_effects(&mut log);
        if monster_is_dead(monster, &mut log) {
            return FightReport { victory: true, log };
        }

        heroes_attack(team, monster, rng, &mut log);
        if monster_is_dead(monster, &mut log) {
            return FightReport { victory: true, log };
        }

        team_tick_effects(team, &mut log);
        if !team_is_alive(team) {
            return FightReport { victory: false, log };
        }

        monster_attack(team, monster, rng, &mut log);
        if !team_is_alive(team) {
            return FightReport { victory: false, log };
        }

        round += 1;
    }
}

fn heroes_attack(team: &mut [Unit], monster: &mut Unit, rng: &mut impl Rng, log: &mut Vec<String>) {
    for hero in team.iter_mut() {
        if !hero.is_alive() {
            continue;
        }
        let damage = hero.damage_against(monster);
        monster.take_damage(damage);
        log.push(format!("{} hits {} for {} (monster PV: {})", hero.name, monster.name, damage, monster.hp));

        if !monster.is_alive() {
            break;
        }
        try_apply_effect(hero, monster, rng, log);
    }
}

fn monster_is_dead(monster: &Unit, log: &mut Vec<String>) -> bool {
    if monster.is_alive() {
        return false;
    }
    log.push(format!("{} is dead :'(", monster.name));
    true
}

fn team_tick_effects(team: &mut [Unit], log: &mut Vec<String>) {
    for hero in team.iter_mut() {
        if !hero.is_alive() {
            continue;
        }
        hero.tick_effects(log);
        if !hero.is_alive() {
            log.push(format!("{} is dead...", hero.name));
        }
    }
}

fn monster_attack(team: &mut [Unit], monster: &Unit, rng: &mut impl Rng, log: &mut Vec<String>) {
    let mut alive_positions: Vec<usize> = Vec::new();
    for (position, hero) in team.iter().enumerate() {
        if hero.is_alive() {
            alive_positions.push(position);
        }
    }
    let pick = rng.random_range(0..alive_positions.len());
    let target = &mut team[alive_positions[pick]];

    let damage = monster.damage_against(target);
    target.take_damage(damage);
    log.push(format!("{} hits {} for {} (hero PV: {})", monster.name, target.name, damage, target.hp));
    if !target.is_alive() {
        log.push(format!("{} is dead...", target.name));
    } else {
        try_apply_effect(monster, target, rng, log);
    }
}

fn try_apply_effect(attacker: &Unit, target: &mut Unit, rng: &mut impl Rng, log: &mut Vec<String>) {
    // `let ... else` : get the effect name, or leave the function if there is none
    let Some(effect_name) = attacker.effect_name() else {
        return;
    };
    if attacker.effect_damage == 0 || attacker.effect_duration == 0 {
        return; // an effect that does nothing is not worth adding
    }
    if rng.random::<f64>() >= attacker.effect_chance {
        return;
    }

    log.push(format!(
        "  {} inflicts {} on {} ! ({} dmg/turn for {} turns)",
        attacker.name, effect_name, target.name, attacker.effect_damage, attacker.effect_duration
    ));
    target.effects.push(Effect {
        name: effect_name.to_string(),
        damage: attacker.effect_damage,
        turns: attacker.effect_duration,
    });
}

fn team_is_alive(team: &[Unit]) -> bool {
    for hero in team {
        if hero.is_alive() {
            return true;
        }
    }
    false
}

pub fn choose_random_monster(monsters: &[Unit], rng: &mut impl Rng) -> Unit {
    let index = rng.random_range(0..monsters.len());
    monsters[index].clone()
}
