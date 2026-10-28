use rand::Rng;

use crate::models::Unit;

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

        heroes_attack(team, monster, &mut log); 
        if monster_is_dead(monster, &mut log) { 
            return FightReport { victory: true, log };
        }

        monster_attack(team, monster, rng, &mut log);
        if !team_is_alive(team) {
            return FightReport { victory: false, log };
        }

        round += 1;
    }
}

fn heroes_attack(team: &mut [Unit], monster: &mut Unit, log: &mut Vec<String>) {
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
    }
}

fn monster_is_dead(monster: &Unit, log: &mut Vec<String>) -> bool {
    if monster.is_alive() {
        return false;
    }
    log.push(format!("{} is dead :'(", monster.name));
    true
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
    }
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
